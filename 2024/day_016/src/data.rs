#[allow(dead_code)]
pub const EX1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"; 

#[allow(dead_code)]
pub const EX2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
 
#[allow(dead_code)]
pub const RS2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#^#
#.#.#.#...#...#^#
#.#.#.#.###.#.#^#
#>>v#.#.#.....#^#
#^#v#.#.#.#####^#
#^#v..#.#.#>>>>^#
#^#v#####.#^###.#
#^#v#..>>>>^#...#
#^#v###^#####.###
#^#v#>>^#.....#.#
#^#v#^#####.###.#
#^#v#^........#.#
#^#v#^#########.#
#S#>>^..........#
#################";

#[allow(dead_code)]
pub const DATA: &str = "#############################################################################################################################################
#...#.#.....#.......#.......#...#.........#...#...........#.....#.#.......#.......#.....#...#.......#...#.#...............#...#.....#...#..E#
#.#.#.#.#.#.#.#.###.#.#.#####.#.#.#.#####.#.#.#.###.#####.###.#.#.#.#.#.#.#.###.#.#.###.#.###.#####.#.#.#.#.#.#.#########.#.#.#.###.#.###.#.#
#.#...#...#...#...#.#.#.......#...#.....#...#.#.#.#.#.........#...#.#...#.......#.#...#.#.#...#.....#.#...#...#.#.#.....#.................#.#
#.#.#.###.#######.###.#################.#.#####.#.#.#.###########.#.###.#.#######.###.#.#.#.#.#.#.###.#.#####.#.#.#.#.#######.#.#.#####.###.#
#.#.#...#...#...#.....#...............#.#.#...#...#.#.#.........#.#.#...#.......#...#.#.#.#.#.......#.#.#.....#.#.#...................#.#...#
#.#.#.###.#.#.#.#######.#######.###.#.#.###.#.###.#.###.#######.###.#.#.###.###.###.#.#.#.#.#####.#.#.###.###.#.#.#.#######.#####.#.#.#.#.#.#
#.#...#.....#.#.........#.....#...#.#.#...#.#.....#.....#.....#.....#.......#...#.#...#.#.#...#.....#.............#.#.#...#.....#.#.#.......#
#.#.###.#####.#####.###.#.###.#####.#.###.#.#################.#.#######.#####.###.#####.#.###.###.#.#.#.###.#.#.#.#.#.#.#.#####.#.#.#.#.#.###
#.#.....#.........#.....#.#.#.......#...#...#.......................#.....#...#.......#.#...#.....#.#.......#.#.#.....#.#...#...#...#.#.#...#
#.#.#########.###.#.###.#.#.###########.#####.###.#################.###.#.#.###.#######.###.#####.#.###.#####.#.#######.###.#.#######.#####.#
#.#.#...#...#...#...#...#.#...#.......#...#...#.....#...#.........#...#.#.#.#...#.............#...#...#.....................#.......#.......#
#.#.#.#.#.#.#########.#.#.#.#.#.###.#####.###.#.###.#.#.#.#######.###.#.#.#.#.#.#.#############.#.###.#.#.###.#.#.#########.#.###.#.#######.#
#.#.#.....#...#.......#...#.#...#...#.....#...#.#.....#...#...#.....#.....#.#.#.#.#.....#.......#...#.#.............#...#...#.#.#.#.#.....#.#
#.#.#.#######.#.#####.#.#####.###.#.#.#####.###.#.#########.#.#.#######.###.#.#.#.###.#.#.#######.###.#.#.#######.#.#.###.###.#.#.#.###.#.#.#
#...#.......#...#.....#.....#...#.#.#.....#...#.#.#...#...#.#.#.........#...#.#.#.#...#.#.....#.#...#.#.#.........#...#...#.................#
#.#########.#####.#.#.#.###.#####.#######.###.#.###.#.#.#.#.#.#.#########.###.###.#.###.#####.#.#.#.#.#.#########.###.#.###.###.#.#######.###
#.#...#.....#...#.#.#...#.#.#...#.......#...........#...#...#.#...#.....#.#.....#.#.#.....#...#.#.#.......#.....#.......#...#.............#.#
#.#.#.#.#####.###.#.#####.#.#.#.#######.#######.#.###########.###.#.###.#.#####.#.#.#.#####.###.#.#######.###.#########.#.###.#####.#.###.#.#
#...#...#...#.....#.#.....#...#.......#.....#...#...........#...#...#.#.#.....#...#.#.#.....#...#.#.....#.#.............#.....#...#.#...#...#
#.#######.#.#.#####.#.#.#####.#.#####.#####.#.###.#########.###.#####.#.#####.#.###.#.#.#####.###.#.###.#.#.#.#.#############.#.#.#.#.#.#.#.#
#.#.......#.#.....#.#.#.#.....#...#.#.......#...#.#.......#...#.#.....#...#.#.#...#.#.#...#...#.......#.#...#.#.......#...#...#.#...#.#...#.#
###.#######.#######.#.#.#.#####.#.#.#######.###.#.#.#.###.###.#.#.###.###.#.#.###.#.#####.#.###.###.#.#.#############.###.#.###.###.#.###.#.#
#...#.....#.....#...#.#.#.....#.#.#...#...#...#.#.#.#...#.#...#...#.#...#.....#.....#.....#...#.....#.#.#...#...#.....#...#.#...#...#...#...#
#.#####.#######.#.#.#.#######.#.#.#.#.#.#.#.###.#.###.#.###.#.#.###.#.###.###.#######.#####.#.###.#.#.#.#.#.#.#.#.#####.###.#.###.#####.###.#
#.#...#.....#...#...#.......#.#.#.#.#.#.#...#...#...#.#...#.#...#...#.....#...........#.....#.#...#.#.#...#...#.#...#...#...#...#.......#...#
#.#.#.###.###.###.#.#.#####.#.#.#.#.#.#.#####.#.###.#.###.#.#.###.#.#######.###.#############.#.#####.#########.#.#.#.#.#.#####.#####.###.###
#.#.#...#...#...#.#.#.#...#...#.#.#.#.#...........#.#...#...#...#.#...#...#...#...#.....#...#.#...#...#.#.....#.#.#.#.#...#...#...#...#...#.#
#.#.###.#.#.###.#.#.###.#.#.#.#.#.###.###.###.#####.###.#######.#.#####.#.###.###.#.###.#.#.#.###.#.###.#.#.#.#.###.#.#######.###.#####.###.#
#.#.#...#.#...#.#.#...#.#.#.....#...#.....#...#.....#.#...#...#.#.#.....#.....#...#.#.#...#...#...#.#.....#.#.#...#...#.....#...#.......#...#
#.#.#.#####.#.#.#####.#.#.#.#######.#######.###.###.#.###.###.#.#.#.#.#########.#.#.#.#######.#.###.###.###.#####.#.#.#.###.###.#########.#.#
#...#.#.....#.#.....#...#.#.#.....#.......#.#...#.......#...#.....#.#.#.........#...#.......#.#...#...#...#.......#.#...#.......#...#.....#.#
#.###.###.#.###.###.#.#.#.#.#.#.#.#######.#.#.#######.#.###.#######.#.#.#########.#.#.#####.#.###.###.###.###.#####.#.#########.#.#.#####.#.#
#...#.#...#.#.......#.#.#.#.#.#.#.......#.#.#...#...#.#.#...........#.#.....#...#.#.#.#...#.....#...#.......#.......#.#...#.#...#.#.#.....#.#
#####.#.#####.#######.#.#.#.###.#########.#.#.#.#.#.#.#.#.###########.#.#.#.#.#.#.#.###.#.#########.###.###.#.###.#.#.#.#.#.#.###.#.#.#####.#
#...#.#.....#.....#.....#.#...#.........#.......#.#...#.#.#...#.....#.....#.#.#...#.#...#.#.......#...#...#.#...#.#.#.#.#.#.....#.#...#...#.#
#.#.#.#.###.#####.#######.###.#.#.#####.#####.###.###.###.#.#.###.#####.###.#.#####.#.###.#.#####.###.###.#.###.#.#.###.#.#####.#.#####.###.#
#.#...#...#...#...#.......#.#.#...#...#.....#...#...#.#...#.#.#...#.........#.....#.....#.......#...#.#...#.#...#.#.....#...#...#...#.#.....#
#.#########.#.#.###.#####.#.#.###.###.###.#.###.###.###.###.#.#.###.#####.#########.###.###########.#.#.#.#.#.###.#########.#.#####.#.#.#####
#.............#...#.....#...#...#...#...#.....#.........#...#.#.....#.........#...#.......#.......#.#.#...#.#...#...#.......#...............#
#.#.###.#########.###.#.###.###.###.###.#.###.###.#####.#.###.#.#######.#.###.#.#.#########.#####.#.#.#####.###.#####.#.#############.###.#.#
#.#...#.........#...#.#...#...#.#.......#...#...#.........#.#.#...#.....#.....#.#...........#.#...#.#.....#.#...#.......#.......#.....#.#...#
#####.#.#######.###.#.###.#####.###.###.###.###############.#.###.###.###.#####.#############.#.###.#####.#.#.###.###.###.#####.###.#.#.###.#
#.....#...#...#...#.#...#.....#.......#.#...#.........#.....#.......#...#.....................#.#.....#.#...#.#...#...#...#...#.#...#.....#.#
#.#######.#.#.###.#.###.#####.#######.###.###.###.###.#####.#.#####.###.#############.#####.###.#.#.#.#.#####.#.#.#.###.###.#.#.#.#######.#.#
#.#...#...#.#...#...#...#.#...#.....#.#...#...#...#...#.....#.....#.....#...#.........#...#.#...#.#.#...#.#...#.#.#...#.....#.#.#.....#...#.#
#.#.###.###.###.#####.###.#.#.#.#####.#.###.###.###.#.#.###.#####.#####.#.###.#.#####.#.#.#.#.###.#.###.#.#.###.#.#########.###.#.###.#.###.#
#.#...#...#...#.....#.....#.#...#.............#.#...#...#...#.#...#...#...#...#.#...#...#...#.#.#.#...#...#...#.#...........#...#...#.......#
#.#.#.###.###.#.#######.###.###.#.#########.#.#.#.#.#.###.#.#.#.###.#####.#.###.#.#.#########.#.#.###.#######.#.#############.#####.###.#####
#.#.#...#...#.#.........#...#...#.#.......#...#...#.#...#.#.#.#.#.....#...#...#...#.#.......#.#.....#...#...#.#.........#.....#.....#.......#
#.#.#.#####.#.#######.###.#######.#.#####.###.###.#.###.#.#.#.#.#####.#.#####.#####.#.#####.#.#.###.###.#.#.#.###########.#####.#####.#.###.#
#.#.............#...#.#...........#.#.....#...#...#...#.#...#.#.....#.#.#...#...#.#...#...#...#...#.#.#.#.#.#.#.......#...#...#.#.........#.#
#.###.#.#####.###.#.###.###.#######.#.#######.#.#####.#.#.###.#####.#.#.#.#####.#.###.###.#######.#.#.#.#.#.#.#.#####.#.###.#.#.#.###.#.#.#.#
#...#...#...#.#...#...#.#...#.......#...#...#.#.....#.#.#.........#.......#.....#.#...........#...#...#...#...#...#.#...#...#.#.#...#...#.#.#
#.#.#.###.#.###.###.#.#.#.###.#########.#.#.###.###.#.#.###########.#####.#.###.#.#.#.#######.#.#####.###########.#.#######.#.#.#######.#.#.#
#.#.............#.....#.#.....#.......#...#...#.#...#...#...#.....#.....#.#.#...#...#.....#...#.#.....#...#...............#.#.#.......#.#...#
#.#.#.#.###.#.###.#####.#.#.###.#####.#######.###.#.#####.#.#.###.#####.###.###.#.#######.#.###.#.#####.#.#.###########.#.###.#######.#.###.#
#.#.#...#...#.#.#.#.......#.#...#.......#...#.....#.#...#.#...#.#.#...#.#...#...#.......#.#.....#.......#.#...#.......#.#.....#.....#.......#
#.#.#.###.###.#.#.#####.###.#.###.#.###.#.#######.###.#.#.#####.#.#.#.#.#.###.###.#####.#.#############.#.#.#.#.#.###.#.#.#####.#.#####.#.#.#
#.#...#.......#.#.#...#.....#.#...#...#.#.........#...#...#.....#...#...#.#.......#...#.#...#...........#.#.#.#.#.#...#.#.#.................#
#.###.#########.#.#.#.#####.#.#.###.###.#.#########.###########.#.#####.#.#.#######.#.#.###.#.#######.###.###.#.#.#.###.###.#######.#.#.#####
#.......#.......#...#.....#.#.#.#...#...#...#...#...#...........#.....#.#.#.#.......#...#.#...#.....#...#.....#.#.#...#...#.................#
#.#.#.#.###.#.#######.###.###.#.#.###.#####.#.###.###.#########.#.#####.#.#.#.#.#######.#.#####.#.#.###.#########.#.#.###.#.###.#.#########.#
#.#...#.....#.......#...#.....#.#.....#.......#...#...#...........#.....#.#.#.#.#.......#.........#...#...........#.#.....#.#...#.......#...#
#.###.###.#.#####.#.#.#.#######.#######.#######.###.###.###########.#####.#.###.#####.###.#########.###########.###.#####.#.#.#######.#.#.###
#...#.#...#.#...#.#.#.#.......#...#.....#.....#.#...#...........#.......#.#...#.#...#.....#.......#.#...........#.......#.#.#...#...#.#.#.#.#
###.#.#.###.#.###.#.#####.#######.#######.###.#.#.#.#########.###.#######.###.#.#.#.#####.#.#####.#.#.###############.###.#.###.###.#.###.#.#
#.#...............#.#...#.......#.......#...#...#.#...#.....#.....#.#...#...#...#.#...#.#.#...#.#.#.#.#.....#.......#.#...#...#...#.#...#...#
#.#.#.###.#.###.###.#.#.#.###.#.#######.###.###.#.###.#.###.#.#####.#.#.###.#####.###.#.#.###.#.#.###.#.#.#.#####.#.#.#.###.#.#.#.#.###.###.#
#.....#...#.#...#.....#.#.#...#.......#.#...#.....#...#...#.....#...#.#...#.#.....#...#...#...#.#.#...#.........#.#...#...#.#...#...#.#.....#
#######.#####.#########.###.#########.#.#.###.#####.###.#.#####.#.###.###.#.#.#####.###.###.###.#.#.#####.#####.#.###.###.#.###.###.#.#######
#.....#.#.....#.....#.#.....#.....#...#...#...#.....#...#.....#...#...#...#.......#.#.....#.#.....#.#.....#...#.#...#...#...#.#.#...#...#...#
#.#.#.#.#.#####.###.#.###.#.#.###.#########.###.###.#.###.###.###.#.#######.###.###.#.#####.#.#####.#.#####.#.#.###.###.#####.#.#.#####.#.#.#
#...#...#.#...#.#.#.#.....#.#.#.#...#.....#...#.#...#...#.#...#...#.#.........#.#...#.#.....#...#...#.#...#.#.......#...#.....#.#.#...#...#.#
###.#.#.#.#.#.#.#.#.###.###.#.#.###.#.#.###.###.#.###.###.#.###.###.#.#.#.#.#.###.#####.#######.###.#.#.#.#.#.#########.#.#####.#.#.#.#####.#
#...........#.....#...#.....#...#...#.#.....#.....#.......#.#...#...#.#.#.#.#.#...#.....#.......#...#...#.#.#.........#.#.....#.#.#.#.......#
#.#.#.#.#########.###.#.#######.#.###.#######.###.#.###.###.###.#.###.#.###.#.#.#####.###.#######.###.###.#.#####.###.#.###.#.#.#.#.#.#####.#
#.#...#.....#.....#.#.#.#.....#.#.#...#.......#...#.#.#.#.#...#.#...#...#...#...#.....#...#.....#.#.#...#.#.#.......#.#...#.#.#.#...#.......#
#.#.#.#.#.#.###.#.#.#.###.###.#.#.#.###.###.#.#.###.#.#.#.###.#.###.#.#.#.#########.###.###.#.#.#.#.#.#.#.#.#####.#.#.#.#.###.#.#####.#.#####
#.#.......#.....#...#.......#.#.#...#...#.....#...#...#...#.#.......#.#...#.........#...#...#.#...#...#...#...#...#.#.#.#.....#.#.....#.....#
#.#.#.#.###.#######.#########.#.#####.#.#######.###.#####.#.#########.###########.###.#######.#####.###.#####.#.###.#.#.###.###.#.###.#.#.#.#
#.....#.....#.....#...#.....#.....#.#.#.............#.....#.......#.......#.......#.#.#.......#...#.#.#.....#.#.#...#.#...#.#...#...#.#.#.#.#
#.#.#.#.#####.###.#####.###.#####.#.#.#.#.###########.#####.#####.#####.#.#.#######.#.#.#######.#.#.#.#####.#.#.#####.#.###.#.#####.#.#.#.#.#
#...#.#.....#.#.#.#.....#.....#.....#.#.#...#.......#.....#.....#.....#.#.#...#...#.#.#.#.......#.#.#.....#.#...#.....#.#...#.....#...#...#.#
#.#.#.#.###.#.#.#.#.#########.#######.#####.#.#.#.#######.#.#.#.#####.###.#.#.#.#.#.#.#.#.#########.#.###.#.###.#.#####.#.#######.#.#.#####.#
#...#.#.....#.#...#.#.......#.......#.....#...#...#.........#.#.....#...#.#.#...#.#.#.#.#.........#...#.#.#...#.#.#.............#.#.#...#...#
#####.#.#####.#.###.#.#####.#######.#####.#####.###.#########.#.###.###.#.#.#####.#.#.#.#.#######.#####.#.###.#.#.###.###########.#.#.###.###
#...#.#.#...#.#.#.......#...#.#...#...........#...#.......#...#.#...#...#.#...#...#.#...#.#.....#.......#.#.#...#...#.#...........#.#...#.#.#
#.#.#.#.#.###.#.#########.###.#.#.#.#############.###.###.#.###.#.###.###.###.#.###.#####.#.#####.#.#.###.#.#######.###.###########.#.#.#.#.#
#.#...#.#.#...#.#.........#.#...#...#...........#.#...#...#.#...#.........#...#.#...#...#.#...#...#.#...#.#.#.......#...#...........#.#.#.#.#
#.###.#.#.#.###.#.#########.#.#######.#####.###.#.#.#.#.###.#######.#######.###.#.#.#.#.#.###.#.#####.#.#.#.#.#######.#.#.#####.#.#.#.#.#.#.#
#...#.....#.#.....#.....#.............#...#.#...#...#.#.#.#.........#.......#...#.#...#.#.#...#...#...#.....#...#...#.#...#...#...#...#.#...#
#.#.#.#.###.#######.###.#####.#####.###.#.#.#.###.#####.#.#.#######.#######.#.#########.#.#.#####.#.###########.#.#.#.#.#.#.#####.###.#.###.#
#.#.......#.........#.#...#...#.....#...#...#.....#.....#.#.#.....#.#.....#.#.......#...#.#.#.....#.#.........#...#.#.#.#.#...#.....#.#...#.#
#.#######.#.#########.###.#.#.#.#####.###.#######.#.#####.#.#.###.#.#.#.#.#.#######.#.###.#.#.###.#.#.#######.###.#.#.#.#.#.#.#.#####.#.###.#
#.......#.#.......#...#.#.....#.......#...#.....#.#.#.#.....#.#...#.....#.#.#.....#.#.......#...#.#.#.#.....#...#.#.#.#...#.#.#...#...#.....#
###.#####.#######.#.#.#.#####.#.###########.###.###.#.#.#####.###.#######.#.#.###.#.#######.###.###.#.#.#######.###.#.#.#.#.#.#.###.#####.#.#
#...#.....#.......#.#.....#...#.#...........#.#.#...#...#...#...#.........#.#...#.#.#...#...#...#...#.#.......#.....#.#.#.#.#...#...#...#.#.#
#.###.#####.#.#####.#####.#.###.#.###.#######.#.#.#######.#.###.###########.###.#.#.#.#.###.#.#.#.###.#######.#.#####.#.#.#####.#.#####.#.#.#
#...#.#.....#...#...#...#.#.#...#.....#.......#.#.#.......#.....#.#.......#...#.#.#...#...#.#.#.#...#.#.....#...#.....#.#.#...#.#.......#.#.#
###.#.#.###.###.#.#.#.#.###.#.#######.#####.#.#.#.#.#.#.#########.#.###.#####.###.#######.#.#.#.#.###.#.###.#####.#######.#.#.#####.#####.#.#
#.#.#.#.#...#...#.#...#...#.....#...#.#.....#.#.....#.#.#.....#.......#...........#.....#.#...#...#...#.#.#...#...#.....#.#.#.....#.#.....#.#
#.#.#.#.#.###.###########.#####.###.#.#.#####.#######.###.###.#.#################.###.###.###.#####.###.#.###.#.###.###.#.#.#####.#.#.#.###.#
#...#.#.#...#...#.....#...#...#...#.#.#...#.#.......#.#.....#.#.#...#.......#...#...#...#...#.....#...#.#.......#...#...#...#...#...#.#.#...#
#####.#.###.###.#.###.#.###.#.###.#.#.###.#.#######.#.#.#####.#.#.#.#.#####.#.#.###.###.###.#####.###.#.###.#####.#.###.#.###.#.#####.###.###
#.....#...#...#.#...#...#...#.#.#.#.#.....#.......#.#...#.#...#...#.#.#.....#.#...........#.#...#...#.#...#.......#...#.#.#...#.....#...#.#.#
#.#######.#.###.###.#####.###.#.#.#.###########.#.#.#.###.#.#######.#.#.#################.#.#.#.#####.###.#.#########.#.#.#.#####.#####.#.#.#
#.......#.#.........#.....#...#.......#.......#.#.#.#.....#.....#...#.#.#.............#...#...#.....#.....#.#.....#...#.#...#.....#.....#.#.#
#.#####.#.###########.#.#.#.#####.#####.#.###.#.#.#.#######.###.#####.#.#.###########.#.#######.###.#.#####.#.###.#.###.#####.#####.#####.#.#
#.#...#.#.......#.....#...#.....#.#.....#...#.#.#.#...#...#...#.#.....#.#.#.......#...#...#...#.#...#...#...#.#...#...#...#.......#...#.....#
#.#.#.#.#######.#.#############.###.#######.#.#.#.###.#.#.#####.#.#####.#.#.#.###.#.###.###.#.###.###.#.#####.#.#####.###.###########.#.###.#
#...#.#...#.....#.....#.......#.....#.......#.......#...#.#.....#.#...#.....#...#.#...#.#...#.....#.....#.....#.#...#.#.........#.....#...#.#
#####.###.###.#######.#.###.#.#######.###.#######.#.#####.#.#####.#.###.#######.#.###.#.#.###########.###.#####.#.#.#.#########.#.#.#.###.#.#
#...#...#...#.#.....#.#.#...#...#.....#...#...#...#.#...#.#.#.....#.............#...#.#.#...........#.....#...#.....#.....#.....#...#.#.#...#
#.#.###.###.###.###.#.###.#####.#.#####.#.#.#.#.#.###.#.#.#.###.###.#####.###########.#############.#########.#####.#.#.#.#.#.#.###.#.#.###.#
#.#...#.#.#.......#.#.....#.....#.....#.#.#.#...#...#.#.#...#...#.....#.#.#.......#...#.......#.....#.............#.#...#.#.#...#...#.....#.#
#.#####.#.#######.#.#######.#####.###.#.#.#.#######.#.#.#####.#######.#.#.#.#####.#.###.#####.#.#####.#####.#######.#.#.#.#.#.#.#.#.#####.#.#
#.......#...#...#.#.......#.....#.#.....#.#...........#.............#...#.....#.#...#.#...#...#.....#.....#.#...#...#...#.#.#.#.#.#.......#.#
#.#######.#.#.#.#####.#.#######.#.#.#.#.#########.#####.#.#####.###.#.#######.#.#####.#.#.#.###.###.#.###.###.#.#.#######.#.#.#.#.#.#######.#
#.#.......#.#.#.....#.......#.....#...#.........#.#.#...#.#.........#.......#.#.....#...#.#.#.....#.#...#.....#.#.#.......#.#.....#.#...#...#
#.#.#######.#.#####.#######.#.###.#.#########.#.#.#.#.###.#.###########.###.#.#.###.#####.#.#######.#.#########.#.#.#######.#####.#.#.###.###
#.#.......#.#.#.#...#...#...#...#.#.....#.....#.#...#.#.....#...........#.#.#...#.......#.#...#.....#.#.......#.#...#...#...#.....#...#...#.#
#.#########.#.#.#.###.#.#.#####.###.###.#.#.#.#####.#.#.#####.#########.#.#.###########.#.###.#.#####.#.#####.#.#####.###.###.#####.###.###.#
#.#.........#.#.#...#.#.#.#...#...#.....#.#...#.....#.#.....#.#...........#.........#.#.#.#.#...#...#.#.....#...#...........#.#...#.....#...#
#.#.#######.#.#.###.#.#.#.#.#.###.###.#.#.#####.#####.#.###.#.#.#########.#########.#.#.#.#.#####.###.#####.###.#.###.#####.#.#.#.#.#####.#.#
#...#...#...#.#.......#.#.#.#.....#...#.#.....#...#...#.#.#.#.#.........#...#.......#...#.#.#.........#...#...#...#...#...#...#.#...#.....#.#
#####.#.#.###.#.#######.#.#.#######.###.#####.#.#.#.###.#.#.#.#####.#######.#.#######.###.#.#.#########.#.###.###.#.###.#.#.#.#.###.#.#####.#
#.#...#.#.#...#.#...#.#...#.#...#.....#.....#.#.#.......................#...#.#...#...#...#...#.........#...#.....#.#...#.#...#...#...#...#.#
#.#.###.#.#.#####.#.#.###.#.#.#.#####.###.#.#.#.#.#.#.###.#####.#.###.#.#.###.#.###.###.#####.#.###.#######.#.###.#.#.###.#######.#.###.###.#
#.#...#...#.#.....#.#...#.#...#...#...#.#.....................#.....#.#.#...#.#.#...#...#...#...#...#...#...#.......#.#.#.........#...#.#...#
#.###.#.###.#.#####.#.###.#######.#.###.#.###.###.#.###.#.###.#######.#.###.#.#.#.###.###.#.#####.###.#.#.#######.#.#.#.#######.###.#.#.#.###
#...#...#...#.#.#...#...#.......#.#.....#.......#.#...#...#...................#.....#...#.#.............................#.........#...#.#...#
###.#.#.#.###.#.#.#####.#######.#.#########.###.#.###.#####.###.###.#####.#.#.###.#.###.#.#########.#.#####.#####.#.###.###.#####.#.###.###.#
#...#.#.#.....#.#.....#.....#...#.....#...#.#...#.#.......#...#.#...#...#.#.#.....#.....#.#.....#.....................#...#.#...#...#.....#.#
#.###.#.#######.#####.#####.#.#######.#.#.#.#####.#######.###.#.###.#.#.#.#.#####.#######.#.###.#.#.###.#####.#.#.###.###.#.###.#####.###.#.#
#.#...#...........#.#.....#.#.#...#...#.#.#...#...#.....#...#.#...#...#.#...#.....................#...#...#...#.....#.#...#...#.....#.#.#...#
#.#.###.#####.###.#.#####.#.#.#.#.#.###.#.###.#.###.###.#####.###.#.###.###.#.###.#.#.#.#.###.#.#.###.###.###.###.###.#.###.#.#.###.#.#.#####
#.#.....#.......#.....#.#.....#.#.#.#...#.....#.#...#.#.....#...#.#.#.#.#...#.....#...#.#...#.#.#.#.........................#.#...#.#...#...#
#.#######.#.#.#######.#.#.#####.#.#.###.#####.#.#.###.#####.###.#.#.#.#.#.#####.#.#####.###.#.#.#.#.###.###.#####.#######.###.#.#.#####.###.#
#S........#.............#.......#.......#.....#...........#.......#...#.........#.....#.......#...#.......#...................#.#...........#
#############################################################################################################################################";