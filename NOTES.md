# LEDIT ( WORKING NAME ) MINIMAL VIABLE PRODUCT: REWRITE D:<

tiny. tiny level editor. three game states: main menu, edit level, and play level.

## TERMINOLOGY: 
  Level: the "game" itself. the thing you make and share. think of it as a pico-8 cart essentially.
  Room : a singular room or stage in the level. think of it as a "map" or something idk

## MAIN MENU:
  just a simple menu, about what you'd expect really. create new level button, play a level button, options button.
  play a level leads to a small screen where you can play installed levels ( levels are searched for in the .config/[ledit]/levels folder, or the Downloads folder )

## EDIT LEVEL:
  small screen focused on one room at a time. you'd have pencil and eraser tools, and also maybe some editor gizmos or something. you'd also have a save button, and a tile picker. 

## PLAY LEVEL:
  self explanatory.

## DETAILS:
  i want to store levels as bitmap files. tile arrangements should be no biggie since im storing tiles in three bytes anyway ( **tile_index**, **tile_column**, **tile_row** ), and bitmap files store images as ( 3 or 4 pixels. im not sure ). if bitmaps store images as RGBA || ARGB, i can comfortably just use the Alpha byte to represent which room and layer a tile sits at ( i want to have a max. of six rooms and ~~six~~ three layers in the end. so that's, what, 5 bits total? with three whole bits to spare which i could use for extra information :OOO ).
  a couple other ways i could store data are:
 -  i could just start every ROOM with a **room_start_pixel** and a **room_end_pixel**. that way i can easily see where a level starts and ends ( for debugging purposes ig ), but also it wastes 8 whole bytes.
    and as for layers i could just use the Alpha byte and store the layer in the first three bits ( 0x00000FFF ), and use the other 5 bits for various bits of metadata.

 -  i can also store each layer as a start and end pixel but that would be even more wasteful. ( 6 rooms, 6 layers per room. one room's (6*2 + 2 = 14)bytes, six rooms are 6*14 = 84 bytes total.
    what am i some sort of AAA developer? smh

 -  i could however not store the room and layer end pixels *at all*, and just end the current room when i encounter a new **room_start_pixel**.however, this way, while shaving off a bunch of needless bytes, also adds on a bunch of complexity and edge cases to handle. layers would have to end when the program encounters either one of: a **start_layer_pixel**, or a **start_room_pixel**.
the last room must have a **NO_MORE_LEVELS pixel**. so all in all i'll need three unique pixels: **START_ROOM_PIXEL, START_LAYER_PIXEL, and NO_MORE_PIXELS**. however, this way of doing things has it's own downsides, or problems i need to figure out. one: i still need someway to store room *and* layer ids. one way i could do that tho is by just using the alpha bit for layer and room id storage.
