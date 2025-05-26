/*
tree object format:
  tree <size>\0 
  <mode> <name>\0<20_byte_sha>
  <mode> <name>\0<20_byte_sha>

notes: here size is content size, and contents of tree objects are the entries

acutally: u8 or byte format in file
tree <size>\0<mode> <name>\0<20_byte_sha><mode> <name>\0<20_byte_sha>
*/

/*
this is the root tree object
tree <size>\0
100644 file\0<sha1>
40000 dir\0<sha1> -> this is also a tree object . it will be stored in .git/objects/ and its content will be 
    tree object format 
*/