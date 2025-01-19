use std::str::FromStr;

use indoc::indoc;
use nalgebra::Vector2;
use soukoban::{tiles::Tiles, Actions, Map, ParseMapError};

mod utils;
use utils::*;

#[test]
fn map_from_str() {
    let no_player_map = r#"
        #####
        # $.#
        #####
    "#;
    let no_box_or_goal_map = r#"
        ###
        #@#
        ###
    "#;
    let more_than_one_player_map_1 = r#"
        ######
        #@@$.#
        ######
    "#;
    let more_than_one_player_map_2 = r#"
        ######
        #@$.+#
        ######
    "#;
    let mismatch_between_boxs_and_goals_map = r#"
        ######
        #@$$.#
        ######
    "#;
    let invalid_character_map = r#"
        ######
        #@!$.#
        ######
    "#;
    assert_eq!(
        Map::from_str(no_player_map).unwrap_err(),
        ParseMapError::NoPlayer
    );
    assert_eq!(
        Map::from_str(no_box_or_goal_map).unwrap_err(),
        ParseMapError::NoBoxOrGoal
    );
    assert_eq!(
        Map::from_str(more_than_one_player_map_1).unwrap_err(),
        ParseMapError::MoreThanOnePlayer
    );
    assert_eq!(
        Map::from_str(more_than_one_player_map_2).unwrap_err(),
        ParseMapError::MoreThanOnePlayer
    );
    assert_eq!(
        Map::from_str(mismatch_between_boxs_and_goals_map).unwrap_err(),
        ParseMapError::BoxGoalMismatch
    );
    assert_eq!(
        Map::from_str(invalid_character_map).unwrap_err(),
        ParseMapError::InvalidCharacter('!')
    );
}

#[test]
fn map_from_actions() {
    assert!(Map::from_actions(Actions::from_str("R").unwrap()).is_ok());
    assert!(Map::from_actions(Actions::from_str("DuLLrUUdrR").unwrap()).is_ok());

    assert_eq!(
        Map::from_actions(Actions::from_str("RddrU").unwrap()).unwrap_err(),
        ParseMapError::InvalidActions
    );
    assert_eq!(
        Map::from_actions(Actions::from_str("RdU").unwrap()).unwrap_err(),
        ParseMapError::InvalidActions
    );
    assert_eq!(
        Map::from_actions(Actions::from_str("RL").unwrap()).unwrap_err(),
        ParseMapError::InvalidActions
    );
    assert_eq!(
        Map::from_actions(Actions::from_str("llurldd").unwrap()).unwrap_err(),
        ParseMapError::NoBoxOrGoal
    );
}

#[test]
fn get() {
    let mut map: Map = load_level_from_file("assets/Holland_81.xsb", 9).into();
    for x in 0..map.dimensions().x {
        for y in 0..map.dimensions().y {
            let position = Vector2::new(x, y);
            let tiles = map[position];
            assert_eq!(tiles, *map.get(position).unwrap());
            assert_eq!(tiles, unsafe { *map.get_unchecked(position) });
            assert_eq!(tiles, *map.get_mut(position).unwrap());
            assert_eq!(tiles, unsafe { *map.get_unchecked_mut(position) });
        }
    }
}

#[test]
fn display() {
    let map = load_level_from_file("assets/Holland_81.xsb", 9)
        .map()
        .clone();
    assert_eq!(
        map.to_string(),
        indoc! {"
            --####--
            -#____#-
            -#._*_#-
            #_._$__#
            #_#**#_#
            #__*+*_#
            -#_$$_#-
            -#____#-
            --####--
        "}
    );

    let mut map = load_level_from_file("assets/Holland_81.xsb", 9)
        .map()
        .clone();
    map[Vector2::new(4, 2)].insert(Tiles::Player);
    assert_eq!(
        map.to_string(),
        indoc! {"
            --####--
            -#____#-
            -#._?_#-
            #_._$__#
            #_#**#_#
            #__*+*_#
            -#_$$_#-
            -#____#-
            --####--
        "}
    );
}

#[test]
fn from_actions() {
    let actions =
        Actions::from_str("uulLdlluRRllddlluuRRdrruRurDDulldldddllUdrruuluullddRluurrdrrurrdDldLrurrdLLuruulldlluRRRurDDullllllddrddrrUUddlluuluurrdRurrrdDldLrurrdLLuruullllllddrddrrUULuurrrrdddlLruruullllddrUluRRRurDDullllllddRddrrUUdrrrruLdllluUluRRRurDDDrdLL")
            .unwrap();
    assert_eq!(
        Map::from_actions(actions).unwrap(),
        Map::from_str(
            r#"
            -----####-
            ######  #-
            # $  $  #-
            # #  .# ##
            #  . #.@ #
            ##$# *   #
            -#   #####
            -#####----
        "#
        )
        .unwrap()
    );
}

#[test]
fn normalize() {
    // Steaming Hot
    let mut actual = Map::from_str(
        r#"
         #      #
         #   #  #
          # #  #
           # #  #
          #   #  #
         #   #  #
          # #  #
        -
        ##########
        #........####
        # $$$$$$$#  #
        #.$......# *#
        # $$$$$$ #  #
        #......$+# *#
        #$$$$$$$ #  #
        #        ####
        ##########
    "#,
    )
    .unwrap();
    let expected = Map::from_str(
        r#"
        #########
        #.$+_.__#
        #.$.$$$_#
        #.$.$.$_#
        #.$.$.$_#
        #.$.$.$_#
        #.$.$.$_#
        #.$$$.$_#
        #._._.$_#
        #########
    "#,
    )
    .unwrap();
    actual.normalize();
    assert_eq!(actual, expected);

    // Sasquatch #41
    let mut actual = load_level_from_file("assets/Sasquatch_50.xsb", 41)
        .map()
        .clone();
    let expected = Map::from_str(
        r#"
        --#####---
        --#@__#---
        ###$_$####
        #_$...$__#
        #__._.___#
        #_$...$__#
        ###$_$####
        --#___#---
        --#___#---
        --#__##---
        --####----
    "#,
    )
    .unwrap();
    actual.normalize();
    assert_eq!(actual, expected);

    // Title: World Cup 2014 (MF8 61st Sokoban Competition, Extra)
    // Author: laizhufu
    let mut actual = Map::from_str(WORLDCUP2014).unwrap();
    let expected = Map::from_str(
        r#"
        --###########----
        --#____*__._###--
        -##_*_*_*_*___#--
        -#_*_**_*_*_$*###
        ##_*__*_*_*_*@*_#
        #___*_*_*_*_*_*_#
        #_*_*_$*__*__**_#
        #_.*#___#__#__###
        ##____#########--
        -###__#----------
        --#__##----------
        --#___##---------
        --###__##--------
        ----#___##-------
        ----###__#-------
        ------#__#-------
        ------####-------
    "#,
    )
    .unwrap();
    actual.normalize();
    assert_eq!(actual, expected);
}

#[test]
fn trimmed() {
    let mut oversize_map = Map::from_str(
        r#"
        ---------------
        ---------------
        ----####-------
        --###  ####----
        --#     $ #----
        --# #  #$ #----
        --# . .#@ #----
        --#########----
        ---------------
        ---------------
        ---------------
    "#,
    )
    .unwrap();
    let expected = load_level_from_file("assets/Microban_155.xsb", 3)
        .map()
        .clone();
    oversize_map.shrink_to_fit();
    assert_eq!(oversize_map, expected);

    let mut actual = expected.clone();
    actual.shrink_to_fit();
    assert_eq!(actual, expected);
}

// Title: World Cup 2014 (MF8 61st Sokoban Competition, Extra)
// Author: laizhufu
const WORLDCUP2014: &str = r#"
    -------#########-------
    -----##---------##-----
    ---##---#####--#--##---
    --#---##------#--#--#--
    --####---##--#--#--##--
    -#-----##---#--#--#--#-
    -######----#--#--#--##-
    #-------##---#--#--#--#
    ########----#--#--#--##
    #-------.*#---#--#--#-#
    #-#-#-#-*-*-$*--*--**-#
    #-#-#-#---*-*-*-*-*-*-#
    #--#-#-#-*--*-*-*-*@*-#
    ##-#-#-#-*-**-*-*-$***#
    -#--#-#-#-*-*-*-*---*#-
    -##-#-#-#----*--.-#-##-
    --#--#-#-#-#-#-#-#--#--
    --##-#-#-#-#-#-#-#-##--
    ---##-#-#-#-#-#-#-##---
    ------#-#-#-#-#-#------
    -----#-#-#-#-#-#-#-----
    -----#-#-#-#-#-#-#-----
    ------#-#-#-#-#-#------
    ------#-#-#-#-#-#------
    ------#--#-#-#-#-#-----
    ------##-#-#-#-#-#-----
    ------##--#-#-#-#------
    ------###-#-#-#-#------
    -----####--#-#-#-#-----
    -----#####-#-#-#-#-----
    -----#####--#-#-#-#----
    ----#######-#-#-#-#----
    ----#######--#-#-#-#---
    -----#######---#-#-#---
    ---#--########----#-#--
    -#--#--##########----#-
    --#--#--#############--
    ---#--#-###########----
    -------#--######-------
"#;
