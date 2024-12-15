#[allow(dead_code)]
pub const EXMPL: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

#[allow(dead_code)]
pub const EXMPL2: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

#[allow(dead_code)]
pub const MV0: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

";

#[allow(dead_code)]
pub const MV15: &str = "########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########

";

#[allow(dead_code)]
pub const TEST: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

#[allow(dead_code)]
pub const TESTR: &str = "##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########

";

#[allow(dead_code)]
pub const TESTR2MV0: &str = "##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############

";
#[allow(dead_code)]
pub const TESTR2MV1: &str = "##############
##......##..##
##..........##
##...[][]@..##
##....[]....##
##..........##
##############

";
#[allow(dead_code)]
pub const TESTR2MVEND: &str = "##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############

";

#[allow(dead_code)]
pub const TESTR2R: &str = "####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################

";

#[allow(dead_code)]
pub const DATA: &str = "##################################################
#..OO......OOO.......#O.#O#..O#.O.O.#..O#.O....OO#
#...OO..O..#...O.OO...O...OOOOOO#O...#...O..O#.OO#
#...OO#O....O.......O.......OO..O..O..O#OOOOO.#.##
#O....#...O..#...#OO.O...#O..O.O#O.O....O.....O..#
#OO..O....O........OO..#O...##...OO.........OOOO.#
#.O..O.#..OO#......O.......O..O.......O.O.#..#..##
#.........#.OOOOO....O...O..O.OO....O....##..#.O.#
#.O.O.OO.#..O..O...#O.......OOO.OO...#O.#.#O.O...#
#...#...OO...#OO..O..O#O.OO......O.O...O....O..O.#
#OO.O...O.O#.#.OO....OO...O#O.O.O....O...##...O.##
#.#..O.O#....O...OO.OO..OO.......O#OO.....OOO.#..#
#.......OOO...O..O.O.O..O..#O.O#OO..O..OO.O......#
#....O.O......O...#O.O#O..O..OO.OO...OO..OO...#.O#
#O....O.O.O...OO.O..O....O.O..OO..#OOO.OO.O......#
#O..#.........O..O.O..#..#.....#O.O....#.O...#.O.#
#OO#O..#....O.O...OOO..#..O.#...O..O...O..O.....O#
#O...O.#..O....O....O.........O....#.............#
##.O.....O...O...#O#O...OOOO...O.O....O..OO.....O#
##...O..#.O.#..OO.....#...OO.#....O.......O..O...#
#OOO#...OOOO....#.#.......OO..O.O...O.O..O..O...O#
#......O.O..O...O.#.OO..O....#...OOO........OO#..#
#.OOO.O###..O.....#.OO...OO#O.O.....O#.OO#.....O.#
#.#O.......O#..O.O.........#O...OO..#O...OO..OO.O#
#..O...O......O.OO...O#.@.........OO..#.........O#
#...##.O.OOOO.....O..OO.......O.#O.....O#OO.OOO..#
#OOO#O..##........O......OO....O..##.O..O...O...##
#OO......##.O.O#.#.....O.O.O.#O.O......O.O##.....#
#.O.O.O..O....O.O.....O...#.......#.O###.#O..OO..#
#.OO.OO.....O#.O..O#OO..OOO..OO...O...#O#O.##.#O.#
#O...#.O#...#..O.O....O....OO.....O...OOO....O..O#
#O.......OO..#O.OOOOO#O...OO.OO....OO........#..O#
#.O...O....#.#.OO...O...O.OO...OO..O.O...OO.....O#
#O.OOOOO....O..O..O....#.O........O....O.OO..O...#
#..#...O#...O...........OOO...O.....O.....#......#
#OO.O...........#.....#.#OOO..O.....O....#O.O....#
#.#...OOO..O.#.O.......##O..OO.O...O.#O.###O.....#
#O...OO...O.#.O......##O......O..O......O.....O..#
#.....#O..O......OOO...O.....O.......O##...#O....#
#......O........#O..#...OOO...O...O....#.......O.#
#O...O.O..O.O#...O##.OOOO.O.O.O.........O....#..O#
##OO.......#OO#.O...O..O.O#....O........#.OO...#.#
#.O...#O...O...........OOO...#..O......#.OO..OO.##
#O.#......#......O#...OO...O....#..#O...#.#...#..#
#.OOOO....O........O.....O..#.O..O#O.......O..O.O#
#O...O...#..O...OO...#O#.OO..O....#OOO......O.#..#
#...#..O..O..OO...#O........#.#..OOO#....O.O.#O..#
#O.OOO...O..O..#.O..O..O.O.O.O....OO.O.O.#.....#O#
#OO..#O.O.#.#.O.....O#.....OO.O...OO.....O...OO..#
##################################################

v>>^^<v<^>^v^>>>^v^^<v^<<>v<^>v^><vv>^v^^>>>v<<>>>>>v>^>^>><v>v^vv><>^^>v^<>>^><^^^v^<>v<^^v>^v<<v^<^<^^<<<^<^>^^v<>v<<^<>v<^>v^>^^>><^<<v>v<<v>><<v^^vv<<^>^><vvv<>^><>>><^>v<><vv<<>>v^>v<^><>v<<>><<<v<<>v<<v^^^^>>v^^<v^^>><>^v>vv>v<>^<v^^>^><v^<<>vv>vv>><>^>^>^<v^<<>v>v>v<>><<^>v^<^<v<v<vv<>v<<^>v>>^v^<v<^v^^>><v>vv<^>v^>><>>^>v^v><v>^>><<>^<^>><<><<v<>>>^<^><v>>^^^^vv^vvv>^vv^^v^^<v><vv>^^>v^vvvv^>^^^v<^v^<<vv<^>><^^^<<v>>><><<>^v<<<><^v<>^>^v>^^vv<<><>^<>>^v<^v><v<vv>>v^^><^>v<>^<^^>v<>><vvv<^<<<><vv^<<>><^v<<v<><>vv<v><^v<<^<<v>>vv<<>>>vv^<<<<<^^<><^>vvv>>><<^v<^v^vv^^<v^vv<<>v>v^v<^<><<<<><^>><><v>v><<v<^>^^vvv^^^<^^<>^>v<<<>>v>^^<<<>>>>>>^>v^v^>^>><^><v^v<<v<^>^v^v^v<^<^^<<>^^>^><^<>>>v<^^v<>>v>^<^>>>>^<vv^<^>^>>^v^v^><<^>><>v<<^>v>>v<><>vv>vv><^vv<v<v^>>^v^v<>><^><>^^<v>^^><>^>vvvvv^>v>^><<>><^>v><^<<^v<<vv><^<v^v>^^v^v^>>>^v<<^^>v><^^v<><>v>^^<>^^<^<^<<vvvv<v>v>^v<vv<v><v>^v<^<<<><<<vv><><>v<vvv^<vv>^<vv<v^<><^vv<<^^>>vv^^><vv<v>^<v<<<<>vv<^<<^^>vvv^v^><^v>v^<<^v^><v<<v<v<^^<vv
>v>v<vv>v>^>^>vvv><>v<>>^v>^vv<<^<<v<><vvvvv><^^^v<^v^><<>v>v<>><>v<><<<>vvv^>>^^<^v>^^v^<<>v>v><v><v>>^vv<<><<^^v<<<^vv<^>>>^^<<>^v^^><>>v<>><^v^v><<v><>><^>>^^^><v^>>vv^>><<<^><v>>v<<<<><>vv<>>>><<<^<^vvv><^v<<^v<v^v>>^<^^vv<<^v^>>v>vv^<><^<^><^<^<><>^v>vv<<v><<v>><vv^>^<^v<vv<<^v><^^>v<<<<<>^^>>^>v>^^<v<<^^^^v^>v<>vv<<>v^>><<v<^v<>vvv^v^<>>v<vv^<v^<v^<<^<<^^>vv>v>v>^vv^^v<>v<v><^v^<v>v^><<^vvv>^>v>v<vv<>^vv^>v<<><<>^^v>>v<>^<<<^<<>v<<>>v>><<^^>v<<<vv>>><^<>>>v><<<><^>^v^^^^^<><>^>^<^^^<^<<^^^>vv<><v>^v>>^^v^v^v>><<<<<^v^<<v<v^<^^^<<>>^^v<^>>>>vv>v^>^<<<>>>^><v<^>vv<vv<^>^vv>>vv^>v^^>^v^<<^<^v>>>^^>^^v^>vv^<><<>^vv<<<v>>v>>v^<vvv<vv>^v^^<<>><v<>^<v>>>>>^<<<^><<v^<<<<>^<^^^v<<v^v<^<><v^v^vvv>^>v<<^^^<^>vv^^>^v><^>^v<<^^>vv>>><^><vvv>v<v<<<>^v<^v>vv^>^v^<^<v^^>><<>^><>v>^^<<<>v>v^><>^^^>><><v>>v^<vvvv^v^>>>>>>>>v<v<<^vv><^<^<>^^^>^<^><v<^^<^v^v^v<v>^^<v><^>^^>v^^vv>^><v>v>^^<v^>^v<v<v^<>^^<><^^<<>^<>v<^^>><^v<>>>^><<^<<^^^v>v>^^^v<<^^>^vv>^v><<>>v<v>^v>v<>v><>><v>vv^v^<>^^v>^^>^v<^>v<>
^<><v<^<<<^v>^vv<<>^v^v^<<<^>>>>^^>^<>v><><^^<v^v^^v^><<^^<>><^v^v>><<<^<<<v<>^v>>>v>><^^v<vvvv>>><v<>^v<<v^>v>v>v^<v>^v>>vvv^^<<<<^><>^>>v>vv><<<<<<><><^><v>v<v^>^v<<^^>^<<^<^v<^><>>>>v^>>><<^^v><vv<<v^>^^^>v<v>><v^^^^<>v<<>v<^<><<vv^>^<><^v>v<v^<v^^^>v^><^<>^^v>v>><v<<v>^<v^<><<<<v>>^v<>vv<>^<>v^v^>^^^<>^<>>^^v>v^^^>v^>v<vvv><<>>^^v>^v>>v><vv>^v^<<>^v^>^^^<>vv<>><><<>>^<>^v<><><<v>>^>vv^<^v^<^><^>^^^v<^v>>^<>>^^<><>>v^<vv^<^<>^<<>vv^<<>v^>>vv<<<^v^<<^v^^v<>v^<vv^<>>>vv<><v<<><<vv><<>v^vv^^<v<^<^^v>>v<vvv>^v<><<><v^>v^^v<^<v<v>v>^^vvv>><^^>>v<<<^v^<^<v>^^<^<^>^>^>><><<^vv^<vv><v><<>v>v^<^v^>>^v<<><<v>v^>^^>v>vv<v><^v<v<<>^>><v<v^vvv><^^^^>^v<v^<<<>vv^<>>>>><^v<<v<<v<^><<v>^><><>^>vv<<>^vv<<v><<>^>vvv><<^v><^<>>><><^<>>^>>^v<<<>v<^v<>vvv<><>>>^><v^<>v<<v^>>v^^<v>vv<>v<v>><>>^v>><<>v>v^v>^>>>vv^^v<>v^>>vv^<^v>v<v>v<<>v>^v^v^^>><v^^^^>v>>v<^<><vvvvv<<^^v>>>v<^<^<>^>v^>v><v<^v<<<>^^vv^^^<><^v^v>vv^<>^>>^^v^>vv><^^>^<^>^>v^>^>><v^v<v^<<v>v<^v^v^<v><v>>v<<<^<>>v<><<v<vv^v>^<^vv^v<vv>v><vv<v
<<>^vvvvvv>^<<v^>>v^><>vv<vv>^<<^v<>v><>^>>>vvvv<v>^>^<>^<>>>v<v>>v<<>>v>^>^^^vv^v><v>^<v>v<v>><>v><<v>^<v><<vv^^v>>v<<>>^<^><v^v>>v^^<^^^v<<<<^><vvv>v<^v><>>><^>><v^>>^<^><><^<<><<>^<v><<v<^<^vvvv><^vv<v<v<<><v^^>^^^v<>^>^v^<<^><v>>v>^^^^<v<<vv><>>v^>^^>^^<<vvv<v>v^vv^>v<^^^<^^v<^v^>^<^>^v<vvvvv><^^v>v^><^>^v^<>>^>>^^>^<vv>v^>^v<<^<<><v<>v>>v<><^>>>v^vv^<^v^^>>^<v<v<^>^^^>>>v^^v<^<v<^^<><<v^>^<<^<vvvv<vv<vv>^>vv^<>^>>v><<<<v>^^>^>^><vv>^<v<^vv<<^v>^v^<>>>^v<<<v>>><vv<^>v<^vv<<^><v<><<>^<>vv<v^v>>v>vv<^><>v>>^v<<v>^>v>>><<<<<v<<v<v<<^^<^><<^><v^vv<<<^v>v<v>>>v<^^<<><>v><^^v^^<>>v<v>^><<>v^>>v><^>^<>vv><v^vv><>^>>v^>>v<vv^<<>>><^^v>^<<<<^<>^><<v^^^>v^v^^vv>>^>v>^^vv<>^<<^^v<v>v^<^<>^<v>^v^<>vvv^^>v<v^<^^^v<>v<^<^<<v^><<v>vv<>>^>>v^vv<^<^^^^v^v>^<<<v<>v>^v>v>vv<>^<<>v>v^v^<^>v>>v<^v<^<><^v^^vv^>^<v<>^<^^<^v>>^<><v<v^>>v>>>v>vv>^^vvv>>^^<>>>^><>^^^<<<^^<<<<>><^>>vv>>>^^^><<><<<>^v>v^<^^^<>v<^<vvv>^<<<^^<>^v><>^^^>><<v><vv^<<^<<vv<><>v<<>^><<^^<^^v>>>>vv><<^<^<v><v^>>><>v<v><<<v<>v>v>vvvv>
<vv>v^vvvv>vv^^^<v^vv<<^v<<><^v^v^>^^<^^^<^<<>><<>^>^<<<<v^^>><<^v^><^v<>^>>^<<^<>v<>v>^>><^vv>v^v<><v^><<<v^^<>>v^<<<v^vv>><<^>v><<^><<vv^>v^^<v><<<^<vv<^<<<^>><>^<v^^^^>^v<v>v^v<^<^v<>v><^>^>><<<<>v^v><>^^><>^^^<>^<>v>^vv^^<^<<v<<^^^^v>v^>^>v^><<^^v^^>^>vv>vv<^v<>^v<^^><>>^>v^^v^^>>^^<><v><v<>v>^>^>><v<<<^><<><^<>v>>>>>>>^v^>vv^^<^<>^v>v>^^<v<<<^>><^vv>>^vv<^<^vvv^<<>^^>^^v^>^<<<>^^^v<<>>^<><^>v>^<<^v>>v^^>>><^vv<^^>^v>>^v^>v><v^^<><><<^v<^><^<<v<<>><>v^<>v<<v^>v>^v>v^^>v>>>><vv>^>^><<v^><v>^><>v<v^<^^<^v^^<>>^^v<v^^<>><^><^v<>vv^><>>>><<>^v^<<<^^<>><v^<v>v^>>^>><v><><^v^>>v<<>>v><v^<><<<^^^vv<<<<<><^^>v<^^<v<>>v^<<<^v>v<^^^vvv^^v^<^>vv^<><>>>v^<^^>^><>>>vv<^^<>^><<vv<>>>^<v^<vvv<<<^^<^<vvv><>v>>v^v<^<<>^vv>v>v^vvv<^^^v<^^<^>v^vvv>v>v<<>><^^>v<><^>vvv^>^v<v<<v>v>v<^^v>v^vv^<^<>^>^<<^v<>><<<^<^^<<>^^^>v>><v>>^vv>v^v<>^v^<<><v>^^v^^v^^><><v^><^v>>>v>^<^^v^<^<^>><>>><>^v<<<^>^^<vv^<^^<>><<^<<v^>^^^<<>>>^<>^<^^<v^>vv<v^v<^vvv<>v<v<>><^v^^<^v^v^^v><^v^v^<>>v<>vv^v^vv^^<^>^>vv^vvv<>v^vv^v<
<>v>^<<<<>^v<>^>vv^<vvv^v<<v^>><<v^v^vv^^^^v>vvv><v<^^^>v^>^><^<v>>vv><^^>>v^<^v^<v>v>vv><v><<vvvv>vvv<^^v<^v<>><^^v<<v<^<^^v<<^v<>vvv<v<^>>^v>v^^>^>^^>v<<>^^v<v^^>^<v^<>^v^<v><<v>^v><vv^v^<v^v^><<>>vv>v<^>v>v^<>>v>>>^^>><>^^vv^<v^^v<^<^^^v^^><>v><v^<v<v^v<^^>^v<v^^><><<vv<v<><<>><^v>>v<v^>>><vvvv^>v>v<<v<<^<vv^^^vv<vv<v^^^<>><^<v<<v^><<<>^<>v^>><^v^>vv^><^^>^<^vv<v>^<^^<>>v^<^^<><<vv^<^<<<>><v^<<^^<<>v>><<v<>v^><^><^<>><^<v<^^v>vv^><^>><^<v><<^<><>^^>^^<<^^>vv^^^vv<<>^^^<<><^>>^<^<v<^^<>^<^<><><>v^^vv>>^><>vv>v><^^v>^><<^v>vv<^>^^<v><v^^^><^^>vv^<>v^<v<^^vv^v>>>^>^>>v^^^<v^v>>^^v<<v^>>>v<v>^>v>^^^^^>v<vvv<<<vv>^^<^^>v>^v>v<vv^<<>v<>v<^v><<>>^v^vvv^^><<v<v>>^^>v<vvv>^>^<><vv>><<<<v<vv^<<<<^v^<>v<>v<^^v^<>^>><>><vv^<<<^v<^<>v<><<<^>^^^>^>^v><^>v<v^^>v>v><<^^^<>^<<><>v<>v>^<vv<^<vv<^^vv<<>>>vv>^v^^v^>v<^<^>^v^>^v>>^vv<<vv<<<^^<^<>>vv<>><><>^v<^v<^vvv><^v<v^vv><<<v^^^>v^v<<<^<<><^^v^<<>^>><^v>^v^^^^v^v>>>^<v><vv^v^^<><<<v^>>^>^^vv^v><<<>v>>>^>>^>>^<v<<<vv>^^^<v>^^v><v<><v^v<><>><<^^v<><^>
v<><>>>v^<>^>v<<^vv<<v>^<>^v^<v>^<vvv<v^><>^<v><v<>v^v^^<>v<><v^>vv^^vv<^><><^>vv^>^v>>^<^v<vv^>v<<v<><v^>>^^^>>v>><^>vvv><<^><>v>v^v^v>^><<^^vv<v^v>>>^<v>><<v^<v^^^v^<v^^v<v^<<^v^<>>^>^^><v<>><><<<^<<<^v>v>>v^v^v<vvv>^^>vv<^<^^<^^^<>v>^>v<^><<<<><><<v>^><<v>vv>>><<v<<v>^><v^v^vv^^>>^v<>>v^^<<v<<<>>>^vvv>v^><^vv<<^v<>vv><vv^^>>>^<<v^vvv><^^v<>v>^><<>>v>><<v^v^<^<>v<vv<v^>vv^<^^>>^<^><<v^<<^>^>>v^^<<><vvv<vv^<^<>^v^<v>^<vvv>^<>^>v^^^^vv<^^<>vv^>v><^<^v>vv>vv><v<^<^v^vv^<v>v^v^v^v>><^<><>><>>><^v>>>^^>>v<><<>><^<><<vv>v>>v<^^^^<><<>vv<^<<<v<<<v^v^><v^<<>^vv^<>v<^v^v>^>>vv>vv<vv><^<><^<^^>>><>>^<<^^v>>vvv>v<>^^><>v^v<<^^vvv<<>^v<^^^<^v>v<v^v^>^><<^^<v<>^><v^<^<^<>vv>v><<v>^vv<^<<vv<<<>vv^vv<vv>>><v<v^>>^vv>v^^>v<^^<^vv<>><><^v>v<<<^<>>v<^^<<<>>^vv^v><><v><<^<<^v^^^v^v<v<vvv>>^<v<^>>>^<vv>^>>v^^v<^>^<vvvv>>^>vv<><<<v^><^^v>v^>^^>>>^v<>>^^^v<<v><<vvvv^<^vv>^<^>^vv<><^><vvv>v^^^>><>v^v>^vv>>><<>v>>>vv<v<v^<vv>v^>>v^v<^^<v>>^<v^v>^>v^v><<<><<<^>^<v^>v^v^^>>v<<>><<v>vv^<^^><^vv<v<<>v^^v^><<>v>
<<^vv>^^^>>>v^v^<v<>>><v^^>v>v^v<v>><vv<v^^<>vv<<>><^<>>v<><vv<>>^<^><<v<<<>>vvv><v^^v<vvvv<>^^^v<^<v^<<<<><v>vv^<>v>>v><<>v^><<><v^>>^^<^<v^^<v^>>v>^<v^^<vv><<^v<><v<>^vv<><v<^v><<^^^v>^^>v^><v^^<v^^^>^^<<>>^><>^<^v<>^^v<>v<<v^>^^<^v>v>^^v><<><^^<^v^^^v<^<<>^^>v>v^<<>><<<v>>vv>><v<<>v<v^^^>^^v><^^<<^>^v>^<^>v<<^vvvvv<^^v<>^>>^><v^<^^^>^^<vv<v>vv<<<<v^^<>vv>vv><<><v>>v>^>^v^vv^>>^^v^><<>>>^><<^v><<<<<^^>vv>v<^v<><<^>^^^^>v<>>v><<v>><>>vvv^<<v^<v><>^>v>v^<^<^>^^^v<<^vvv<>vvv^<<v^v^><<<^^^<^^^v>^>^v>vv>v^><<v<<<<^^>>v<^v<<<>>v<^v>^vv^v<<v<<v><v><<^<^v^^^vv>^^v^<>^>>^<^<^>><<^><>>v<v^^v<<^<<<>>^v^vv><>>^<v>><v^<^>>v>^^v^<^>>^>vvvv>><<>>><v>v<<^>v<>^<v>^v>>v<>v<vv>^v>^>>>vv^v^^^^^<vv<v<<v>>v>><v<^<<v^>v<v^<<<v<<>^^>^v^^vv^<^<^v><<>^>^v<<^^>>^^v>v<^v>>^v<^^^>^><^>^<><v><>>^>vv>>>v<v^^>^v^v^><^><v^^^<v><><^<>^>v>><<<^^v<><<>v^v<^>vvv<>^>vv>>><><v<<^<^<<^v><v^<v>^><^^^^v^>^^<<v^<<v^v^><>^>^>>v^<^>v<>v<^^<>vvv<><>v<^<^<<^^>^<><^>v><vvv^^v<^>vvvv>v>^vv^^v>^<vv^<^><v>^v><><><v<<v<v^<^v>^^<>^>>>>
^v<<^>>><v><v<^>^^<v<vv^<v<<vvvv>>^v<v<^<>v>><>v>^><vvvv<>^^<<><>>vv<<^^^<<^^^>><^<<>vv<v^>^>vvvvv<^<^v><v^<<<>>vv^^^>^<<<vv^<^<^v>>^v>v<><^v>^^vv>^<><^^^v<><<vvv><<^>^^<<^^>><<^^<^^<><<>>^<<vv<<>^^^^v><>v^>>^>^<>v^>>><>^<<vv^^>^v>^v^>v^>^<^v^^^>>^vv<>>>^vvv>>^vvv>^<vv^v^^>>><v^v^v<><v^v<<>>>^^>^><vvv^^v>vv<>v<vvv>^<<><^>>^<v><^^>^^<<v^^v<>^^>v<>>><^v^^>>^^<v<<<<^><>v<<>><><^>>^<vvv>^v^vv^vv<^<^>^>v>><v>^^<<v>>><>v^v^>vv>v<>v>v^^<<^<^vv<>^<><vv>^^vv><v>>^^^<<<<>vvv^<<<>><><^^^vv^vv^<>^><<<^vvv<<><>^v<>>^<>>v^^>v<<<^vv<<><v^>>vv<vvv<>vv><^<>^v<>>^v>^<^^vvv<><<>^>v^^^^<>v^^<vv^^^<^^v>>>vvv^^^<^>>^v>>v><v>^v>^<^<^v^<>^^v^v^^^v^<^^<^>>v<^<>^>v^v>>>><>vv^vvv>^>v<<<<><v><>><>>><<v^<vvv>^^vv><^^>>^vv^v^<^^<vvv<<vv^^>^>v^>>>^^>v<>^>><><v<v<>^^v<^vv<vvv^vv^><^<^^>><<<^><v<^>>>v>>>^v<^<>>v>^v^<<v<>^v^v>v>v>>><v>v>v<><<><^v<^v^^>>^vv^^^v>>^<^v><^><<^^<v<>^^^<<v>vv<<vv>^><v<v>>>v^>^><vv^v<v^>^^<vv>>>^vv<v<^v<v<vv><^<vv<<>v<>>v^v^<><>v^^>v>><^>v>v^<>v>^v^<<>>>^^v^^>v<<>v>^>^v^vvv^v<v<^^>v<^^<vv>v>^
^^v^vv^>>>v<<^^<v<^>^^<vv>><^>>v>>v<^>>^^>vvv>^>>vvvv^<<v<<^>>^^^^^^v^<^<>v<^>^<^<v>>>v>>^>^vv<>v<<v^^<v><^><^<^v^<^v>vv^^<>>>>v^<<vv^^>v<^^v<<^^<>^<vvv>>^>v><v<^v<<vv^<v^v<v<vv^<>>>v^^v<<vvvv>v^v><<><^^v^><v^<>>v>>>^<v<vv^v^<v<^<<<^<^v>^<<^<v>^<v^vv><>>v<<<^>^v>^vv^v^v<vvvvvvv^<^^v^v><>^<v><^vv^>>v>>>v<v^^>><^><<>>>^>^v^v>>^<v>v^<>v><^<v^^^<^^<v>v>vv>>>>v<v>^v>^>>>v>^<><<vv<^>^v<v>vv>^><^<>^>^>^>^>>^<vv<v<>>vv>>vv^<<<<<<vv>><><^>><>^><>v>v<^^<v^>vv<>>^^<^<<<<<^v><>^v^>^>>v<v^><^>^v>vv>>^><<v^^^><><^v<v>vv^v>^>vv^v<<>v<<>^<<>^<>>><^^<v>v<v<>vvvv<<<<v<>^<<<<>><vv>^^<^^^v>^^v<v<^>v<<>^vv^^<>v<^v<<>><^<>>>^<v^>>>>^<><<<vv^v<<v<>>>v^^v<v>^^^v^^<><v^vv><^>><^^<<^<^<<><>^vv>>v<>^v<<>vvv^vvv^v<v^<>>><<<v<><<>vvv>^^v^<>v<vv<v><v><>vv<^>^vv>^v^^v^>>>^>^<vv<v>^><>>v<^^><<<<^vv^><<<^>v^<<^>v>><<<>^>>^><<><^v<><^vv^vv<>v^v<v>^>>^vvv^^<>vvv<v^v^<vv^^>>>v<v<v>><v^>vv>^^v<v^^^v<<v<^>^vvvv^><vvv<><^<>^v<<^>^<^><>^^v^^>v<>>v^<>>><><<^^vv>vvv^<>^<v<>><^^^<><<^^v^<v<<^><v<^vv<>^>>^><^v><>v<><<<>v>v^v>^<<
vvv<><v<<v>vv^<><^<>^v^v<<v<^^^^^<>^vv>^<<>^v><^v<^v>^^v^^^^^>v<^<>^>vv^^><<><^<vvv<><v<v^>vvv^v>^^><>^><vv<^<>vv^<<>>>^<v^<vv^v<v<<>>^><v<<^v<^<vv^>^>^><>^<v^^<v^vv<v^>^<v><>^>^v^<^<vv>v>v>>^<^>><^^>^v>^v>^^v<<^^^><>v^^<<<^>^<v<vvv>v<>>^^<v>>><>v>>^>>vv^v>v>>>v<>>^^^<v>v^><^<<vv<>>^><<^v><^>v^<<vv<v^^^>vvvv<^^v<v>>v><vv<>>v^<^<v^>v>v<>^vvv>v<vvv<^^v<v^<<<^v<v<>^^^^<<^<>^><^^^>>^<>v<<>>><^<><<<<vvv><<v<^^vv<vvvv<^^^>^>>><>><<v>^vv>v>^<>>v<v^<vv><>>^vvvv><v<vv<v^>v^^^^<><>^^v>v^vv^<>^<>^><>><v^>><<vv^v^>^>vv<>vvv<<v^^>^>^^<^v>^<v><v>>vv^^v>v>v^>>^^v>^<v<>^>><vv^v^v^<<vvv<>><>><vv>v<>v^>^><<<<>^v^^<^^vv>^<^v>^v>v<<^^<^^^>^^^><^v^<v<^<v<v<^>>v><vv^<v<vv^^<>^^vvv^v^vv^^v^<vv<<<<>>v>^^>v^^<<v<><<^>>^><^v>v^>v<<<^>^v^^^^<^<>^<>>>^^<<^>v<^^<<><vv^^><v^v<^<<>><<<<^<>>^^^>^^>>>vvv^><^<^^^v>vv^^>v<v<^>v^<v><>>^<>>v<^v<vv^<><v><^v<^vv<<^<<^v><^v<v^><<v<^v>><^^v^<^v<^^^<v>><v<^<<^<<^>>>>v<>>><><v>^<v>>>v><<<>>v<<>>>>v>v>^>^>^<<>><>>>v^<vv>vv<^v<<<>v<>>^>^^>>v><^>>>^vvv<v^^vv^vvv<>^v^vv><>vvv^v><^^
v^v<^<>^^^vv<>^vv^><>^><v^^v>v>>^>^>vv><><v^<^v><<^><v>><v><v<^^<^v<><v<>v>^><>v^<v<<^^^>>^^^>><<v^v<><<<<<^v^^<>vv<>^vv<v>>^>^v^<^v^^vv<vv^<<<v^vv<^v^<^<vv<<>>><<>>vvv<v^^<^^^>><><^^^>>^><<<<<<v<<><v<^>^<>><v<<>><<v^^vv^<^vvv<>^>v<<vvv>^>v<>><^^<>v>^^<v<>v<><>vv><v^^>^<^<<v>>v^>vvv>>vv^>>>>^<<>vv<v<<>^>v^^>v^v<v^<<>>>v^v>v>v>vv<<<v><^^v^^<^>^>><v<^^>v>v><<><v<>>^v^>^>^<<vvv^><^<<^<>>^<<>vv>>><>>>^<^<v<>vv^>^v><>v^>vv^><<v<<>^v^>>^^^v^>v^^v>><^<vvv><^^v^><><>>^<vv><<v^<>^<>>>>^v^^^>^>^>vv>^<><<<>^<^v>^v^v^v>>>>^v>>v<^vv>^^<>v^v<vv><v>vvvv>>>^<^<>>>^v<v>>>^^^<vvv>>><<^vv<<<>vvv^>^vv>>><><vv<^^<<>^v>><v<v>>^vvv^v^>^v<<^^<^^v^v<^>v^^v^v^<^v<>v<>^<>>^v<^^^^^<<>^<v^<>>^v<^<^>vv>>^>vvv>^^<v><><<v>v>><>>><^v^<>^v^vvv><v^vv^vv^<^^>>v>^^<<^v<<<^^v<^>>vvv>v<<<>>><>>^v<>>>^^^>>v<vv^<<><v>v><vv<<><>^^vv^^<<^>>v^>^^^>>>>^v^><v^<^^v^^^<^v>vv<<v>v>>^v^>^^>><vvv<<v^><^v><>^<<^>>^><v<><<v^>v^vv<v^^^^^<>><^^<v<^<>vv<^<^v>^^v><><v>^vv>vv<^>>^>v<^vv<vv<<<<>^<v>^<<^>^^^<<>^><>v^^<^>>^^v<v^><^>v><>^<>v^^<<^
>^><^vv<v>v^^^<>>^v^>v>^v^<<v^>>>v<v<vv^<<><<^^v<<^v>^v<^<<<<v<<<>v>^>>^>>^>><^>>v><<v^<>v><<v>^><>v<<v>^v>^>^>^^^>>v>v>^^<^vvv^<^^><v^><>><>v^>>>^>^^>>><>^v<v^<v<^^^v>v>v<v>>vv<v^^<^v>^^^><>^>vvv^^<>^<^<vvv^v<<>>vv><v<vv>vv><<<<>v^^>vv>v<^><>v>>^>>^<v<v>>^^v>^v<^>^v>>^v><>^v>>v^v^^><<><^>><v<>^v^vvv<<v>v><<^>v<>^<<<<vv^>><v^^<<vv<^^>v>><v<><^>^>>v^<vv>>vv^>^<^<^>^v^<<^^^v>>><vvv>^><vv^vv>v<v>^^vvvvv<>^^^^v^v>vv^^vv<^<><v<<v>><>v^v<^>^vvv^<^<^^>v^<^v>><>v^^^>v^^<>>>^<^<>^^<<^v^>v<v<v^>>^^^>>^^<<<>^>^<^^>><>^<^v><v<>><>^>^<>^^^<><v><^<^v^^>^vv^^<>v^>^>v><<v<>^^v^<<^^<^v^v<<v<^>><>vv^v>^<>^^v>>vvvv<v>vv<<><>^^^><^^v^><vvv>v<<vv<<^<vvvvvv>v>vv><^<>v>>>^v<>><^><>v^<>v>^>v^<^^^<v^<v>v^<v<<v<v<^<v<^>^^<<<^v><v<<>^<<>v>^^^v<>v<^v><^^^>>>>v>vv<<^>v<>^v<^v^><<^^><^>v^v^^v<<<><<v>vv<v>^<vv^>v^^^>^v<><^<v^>v^<vv^<^<<<^v<v^^>>>v<v<^><^<><v><<^>^v<<<>>>v>v>^<<><^<v>><v><vv><><<>v<v>v^>^<v^<^^^^^>>>^v<^^>vv>vv<<^^>><<v^><^v>v>v<<<>>v>v<<<vv<vv>^v>^^vv>v^v>v>^^^>^><><v>vv>v^>>^^>v^^>>v>><>v>v><^<<vvv
<v>v>><^<v^^><^<vv^>>v>v^<vv<<vv><vv^>>v<^><v^<v^^<><<<v><v<v^<^>v^>>^^>v^v>^>v>v>>vvvvv>>>^v<><<^^vv>^>v<v^vv>v>v<vv>><v<vv<vv>vvv<<<^<vv<<^<>v<^>v>v>^>v>^v>v<^<<v>v><<>^vv<^v<^^>>^^>v^>vvv>>v^>^^v^><^vv^^v>>>^^v>^^^vv^>>v>^<>v<>v><<>^v<^v^<>vv<^><v^>v^<v<>>>v><v<v<v^vv><^>><v>^<><<^<^^^<<v><<<vv<<><><>v>>><v<<<^<><^>v>^><<<^>v<><<>v<><^><<^^>vv<<vv^>v<>^^v>>v<vv<^^<>v>v>^<^^v>^^>><^^^^vv<^v^^>v>^^>><<v><^v<>vv<^vv>^<<^<^^<v>^^<>v<<<v<<^^^v<v>v^>>><<v>v<<v^>^<><v^>^><<<^^<^^^v<v^<<<>>>^^^<^<vv<^^v<^^<vv>^>v<><^<^><>>>><^^v>^v<<^>><^vv>^<><><^^vvv<>^<>v><>v^v^^v>v>v><<<^^<<^<>><<v<<>>v^^>^^<^v<^^>^^^<^v^^<><^>vv<<^<v^vvvv<^v<vv>^<<><^^^v^<>v>v<^v<v<v>v>^>^<<>>^v^><>v^^>^>>>>>v>v>^<^^>>>^^<<>v^vv<^^^v^^v^v<v>v<<^>v^<>>v^><>>^^>v>vv>>^<v<v^vv><vv><vv><<<<v^vv<^>^<v^>^>><^^><v^>><v^>>^v^>><v^>>^^>v<>><>>>^^vv^^>^>><><<<v>>>^^^^>^vv^^<^v^^^^><v^^^^<<<v<^<>vv<<^>vv<<^>^v<vv>vv<^<><<<v^^v>>>>><v^<^v>v><^<vv<<>>v>v<v<<v>v^><>^>^^<<v><><^>v>v>^^<^^^^<v^^<<><<>>^^><v<^v<v><^<>>>>>v^v><<>v>^vv<^
v<vvv>>v^<^<<^^>>^>v^<<v>v><^v>v^vv<<<v<<<<^>^^^^^vvv><v^<>v>v<^><^<<>^^v<vv>>vv><^v^vv^v^v^^v>>v^<^^<<<^>vv<v<v^><^<<v<^v<><<^>><^^<>>>><v>^^^^^v<>v>vv>^<v<>^^vv<<>^<>^<>>^<>><vv><^<v<v^v>><^>>v<<vvv>>vv<v^>^<^^<>>v^<<^<<>^>vv^>^v<v>v>v^<v<^^><>v<^^v<^^^^<^<^v<v^v^^^<>vv^<<<^^>><<>^>v<<v<v^v>^v^<v>v>>^v<v><>^^<v<<v^>vv^<^>v<^v>v><v<<>^vv<>><v>><>^>v<<^vv><^>>><>vv^^^^<v<<<v>^<>><<<^v>^v><v<^<>^><>>>>v<>vvvv^v>^^<^<^>v>v<><>^v^>vv>^>^><v^>>><vv<^<vv><>>>v<>v^<vv^>v<^><>^<>^^<>^>><v^vv<v><^^<>v^>>vv^<<vv^v>^>><v<<v>v>^^^>v<<<>vv>v<^v<<<><v<^vv<vvv^v<^^^<<<vv<^>><<^>v>v^v^^v^v>v<<<<^>>v<>v^v><^<^^v<vv<v<>>vvvv^>>>v>v>v><vv<<<<>>^vv<^<><v<^^<>><^<>^^>vv^><v^vv<>>v^^v><<^><^^<<^^^^v<^v<v<<v^>vv<<><v<>>^^v>v>>^^^<<>v<v<<>^<v<^^>^^^<<^^<v>><^<^<><^<^v^>vv<>v>^<>^<^v>>v<>>>>v>vvvv^<<<^^><>^v^><>^>>^<<>^v^>>v^v>^^<v>v<<^>>^>^>v>^<^>><<^>^v>>>v<>vv<^>^^v><<^v<<^v>vvv>^v>^<v^^<^v>^<^^<v<<v<^<<^^<v^>>v<>^<<^^<><>v<v><<<^>><<<^<^v><><>v>^>^v>v<^vv<v^>^^^^>^^><^v>^><<v^<v^vv><^<v<<>^^<<>^<>v^>><^>^
>v^vv>vv<^^^^><v<v<^<^v^><v><vvv^><v<v^v><v^^^v^>vv^^<><v<<^vvv<<>^^vv>vv>^^<>>>vv^>^>>>><<<>v>>v^<<^^^v><^^v>vv>^^>v>>v<^<^<v^<>v^^^v^^^vv><>^^>v^<^^<<<^^^>>>v^^v<>>^v>vv><^^vvv<>v^<<^<^<<^<v><<<<v^>><v>^>><^<>>v^><v<^>vvv<<<v>v<v>v<v^>vvv^><v<^^>><^<^>>>>>^^v><v>^^vv^<v<^<>v>^<<v>^v^v^>^>v<>^<^>v>^^>v^^>>^<<<^v<>^<><^v<>v^<^v^v<^v>>>>>^<>^^<v<v><>^<<v>><><<<v<>^^>^<>><^<^>^<<<<>^>^v><<>>><<>>^><<^v<^v><>><v^<<v<>>><>^>v^^<v^^>^<<<>>^^>^><>v<<v<>v<^^>>v><>^<^^>vv<v>^<>v>><^vvv^^v^^>^<<>>^<v<^>^^v^v<v^>v<^>^v^vv^^vv<v^>^^v<^<v<v^<<>>>v<^><>><^vv<<v<^<^v><><^<<v>>^^^v>v<><vv^<v<^v>>>^^^^^^v>><v^>v<<>v<<<><vvv^^v^v<v^v>>^<v<<>>v>^<<^^>^v<v^<^v>^><><<^vv^<<<>>^>>^^^^vvvv<>^>v<v^>^vv<vv^<<v^vv<v>>>v<<v><vv<<>^><><<v><>v<<>v>>^^<<^<>^<^^>v^<^v^^^^^^^v^^>v<>>^<><<^^>vv^^v<^^v<^^>>v<^^<v<^>><>^v<^<v<^<^<^<^>^<^v>>v<>><<^<>^^>v^>>vvv^<^><<^v>^v^^vvvv>^>v<<v<v>>><v>^^>>>>^><^<v>v^v^>v<v>^^^vv^>vv>v<^v<><^<vvv>><>>>vv><v><v<v^><><^><<vv>^<>v>><><vv><><^>^<v^v>^<v^v<^>vvvv<^^>^>>v<<^<^<^<v^v<^^<v
>vvv^vv>v<>>v<<<v<<<>><v<v<^^v^<>^v^v<>>>^>^>^v<^<<<v<^>>vvv<vv>v>>^<v>v><v>^v>^v<><v^v<^^vvv^v<>><^>v<>v^<<<>v>v><<^>vv^><^v><<^^^><^<^>>>>v<>vvv^<v><>>>>>^<>^^>vv<>><v^^<^<<>>><<>^<<^>>v^v>><v<v^<^^<><><^v<vvv^v>>>^^^>><<<<vvv^v>><>>>v>^v>>^><>^<<v<^>^<>>vv<v^v<>^>^v^<>^<^^><<<v>>^v<v>><v<v^>^vv>>>^vv^v<>^^^v<v<<^>vvv^>><^v>^><^>>v<^^v>^<^<^^v<^^<<^><v<v><^^<^<v^>vv<<><v^^<<^>v^>v<><>^^v^<vv^^<>v^<>>^><vv>><^><^^>v>>^<^<>^v>><^v^v>>>v^>>>^v<^<>>^vv^<>><>v<^<^<^^><^<vv^<^>><>vv^v<<>v>v>vv>^<^<^>^<>^^<>v><><><^<v><v^^<^v^<<>^vv<v^v>^>v><>^>^v<^>>vv><^<^>>^><^<>><^v^^v><^^>v<>>vv<v<v^><^^<v<>>^<>^<v<<^<v>vv<^^^>>><<^<v>><v^^>vv>^<^^<^v^<^<v<<>v^^^>vvvv^><>>v<v><>^>^v<<><<>^<vv<<<v<>><v^>vvv^^<<^>>>vvvv>v><^<>v<<^v^^>^>v^^vvv^v<><>><v<v>><^><^v>^^^<<^<v<<^^<^v<^<<vvv>>>>vv><<v>^>vvv^^^<>v^v^vvv<>>^^>>^^<<^^<<<<^^^>>>>^<^<<^><>^<^v^>><^<^><v<^<>>vvvv><>><^^>v^v<v^>>v<v>v>^^v^<>><><<v>>^v^^v<<^>^v^^v<<>^>^>>^<<v><><><<>^vvv<>>^v>^^>^<>v<v<^><^^<>^>^^v<>><<^>vv<v<^^<><v<^<<v>><><v<<>^>^^^>^
><>>v<<<<<^<v<vv^>^>v<v^<><^>^<>><^>^^>vvvv<>^>^^^vv>v<>v<<>>^<>v^v^>^vvvvv<^v<^^^<vv>^v>^>vv<^^>^>><>v<v>>v<v<><<vv^^>^^v<^>v>^<<<v^vv<<v^>v^>>^^^>>^<^>v^<^<<>>^>>v>vv>^<v<>^><v^<>>><<>><^>^vv^>^v^v^<<<<>^v><^>><>>v^^^>v<<>>^<>^>^<^v^v<^vv>^>v<v^>v<v>^^>>vv^^^<<v^<v^<^v<^v>^>>v^^vv<v^>>^^^v><^^<><^v<v><v<<^<^>^<><<<v>v>>v>v^><v>^><^><>><>>>vvv><>^^>>>><<<^v^><>>v><^<>>><v<v>><^v^^^>vv><v>vv<<^^v^>^^^>>>>v<>><^>><^<^vv^><<^v^v<><vv<^v^<v^<^<^<><<<v<^<vv<<>^<>v^>><v^vvvvv^>^v^^<^^^><<vv^<v^v^>^vv^<<vv^^v^<<<<><<vvvv<>^>><>^>><<>><v^^<<>^><>v^<^><^><vv^^>v^v>vv>^^vv^^^<<v>>>v<v<^>^>^v<^vv^<>>^v<^^<v^>^<^<<>^>^^vvvv<<v>v<<^<<>v^^^^^v<vv>v>^v>v>^^>v^v^<^v<v>^vv^v^^v>^<vv<^>vv^v>>^>^<^>>v>v<<^^^>^^v^><v<>^vv>^vv^vv>v^><<><^^<<^v>^^v>>>>>^v<v<^v<^v^>v<<v<<>v><v<><vvv<<vv>vv<^v>v<^v<>><<^^vvv<>^>v^>vvv<v^v<v<v<>>v<>><<<v^^<^^<<<<^v><<>^v^<v><v<^^vv^^^>><<><^v^v<<<<v>^<>^<>><^v><^><^^><>><<^<<^>vv<<<v>v<^^>^^^<v<>>v<v<^v<^>>v><v<^>v^^>>v<^v^>>^<<>vv^^^<v>>>^v<^v>>^^^^vvv><^<^^<^<v^>v<v>^>v<^^<
^^v<v<<vv<^v<^^^^>^<><v<^>><>v>v<>>v>^<>>^^<<vv<^v^^^^<^<>^^^^v<>v<<^^^v><^<><v<vv>>^<^^<v<^>><<vv^<v<<><><v^v^^><^<<><^v>>v>^v<^<v^^v><<>>^<vvv<vv^^<vvvv>v^<<><>>><>^<^v^v^v^^v^^<<^<<>><vv>v^v^v>v^<v><v^>>>^v<^v><>v>>>v><v^v^>><>>v><>^<v>>^v>^<v<v><<<<^^<<^>>^^>>^v<^>^><>><^<<v<^>>v^<><<><v><^<^^^<vvv<^<<<<>>^vv<<vvvvv^>><>v^<v>vv>>v^>^^v<v^<v^vvv<>^>v<><^>^><<<<>>^>^<^v^^^>v><^<>v><<<v^<>>v^<v>^<<v<<<v<>^><v<vv>^>vv>v<^^><^<<vv^>><^<vv<^v>>^>^<<v^^<><v><>^>vv^^<vvv^v^^<><^^>>v<<<>v^>^^<<^<><v>>>^^^><v^v><>v<>v^v<vvv^v<<vvv^>vv<<>v<>v^^<vv<^<>>><vv>>v^<>^^<<>v>v^<>^<<v^<>v^>v^<>vv<>^<<<>>^vvv>>v<v<>v<<><<<><v<<v^v>>v>>^v^^v<vv<v>>v<vv<v<>v^^^>v^^v^^><^<^<^<v^<>v^^<v^><^vv^^vv^<vvvv^^^v<<^>v>><^v^<^vvv<v^>^<v<vv^v^v>^><^^v<>^v^>v<^<<^<^v><^vvv^>v<v<>^v<^^v<>>v<^v>^vv>v<<<v>>^>v^<<<>^<^^^v>^^vv>v<vv>>v^><>v<<>><vvv^v^^vv<<^^v>><<v>v^<>><<<^^>vv^v<>v^^v>vvv>^><>v<<<>><>^^><<<>^>>^^<^v^^v><^>^><<v^v<^^>>v>^v>v>>^<<^v><<>v^^v^<^><><<^^>^^<^<^^<<<>^vv^>><<v<>^^vv^>v>v><^^>v><^<^^>^^^<>^v>vv
vvv^>>vv^v>>>>^<<v>><vv^^vv<^^^><^><>v<>^>v<>^v>>v<v<>vv^v<><<^^^v^^<<^<<><^^><^<vv><>^^v^>^>^>^<<><<><<v<>v><v<vv>>>>v<^v<v><>vvvvvv^<v^v><^<><v<^<^^vv>>v><<^<v<<>^>>><v<^>^^^^>v<<<<>>v<<vv^<vvv><>^<>v<>><<>v^^<^>^>v><v>^<<v<^v>v^^<v<<v>vv>^<<>>^<^>^^<^>v<>>v>v><^vvv<^<>v>>^^>^v>vv>^v^<^^<<>^<^<v<v>>^><><><>v^><>v^^vvvv^<^^^^<^>^v>^vv>>v<<v^^<>><>vv^>^v><v<>^<v^<>v<<^v<<v^v<>vv<v>v<>^vv^v>^<^>v^v^<<<<v^>vv^v<<v^><^><^^v>^v<^^<>v<>>>^^<<<>>>v>^^^^^<>v^vv>^<^<^^^vv>^<v^>v>>>>^>^<^^vv^^vv^v>vvv>>>><<v<>vv^vv<<><vvv^>>>^^^v><v<><v><^<>^v>^>>^^>vv<<v><>>v^^^^<v>v<><v^^^^<>^>^v>^><><^>>v<^>^vv>^>v^<^^>v>^<v><vv>^^v><<><^><>>v<^><v^^v<^>^^<<^><><v><>>>^<vv<<>v^<v^^>>vv>>v<^>v>>>><^><^>v^<v<^<v^>^><v>^v>>^>>v^vv<<vv<<>vv^^>>v>vv<><>vv^<<vv>^^^^v^^<vv><vv>>><^^<^^<><^v<v>v>^^^v^>v<><vv>^<>^>v>>^v>v^>v<<<<vv^>v>^v<v>v>>vv<<^><><<<v^^>^><<<^v<<^^^<<v^v<v^v>>v<v>vv^><v^^<>^^<^>^vvv>v>>vvv<>v>>^>><>^v>v<^<^^><^>v<>v>v<<>^^^>>^>>>^^v^<<^>^^<^>>>^^^^^>^v<^v>^^<<v>>^<^<v>v<>>>^v^<v>v^^<^><<v^v^v^<<v>";
