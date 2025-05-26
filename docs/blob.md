# Blob Object

## Blob Structure
blob size\0content

## Read Blob Object Algorithm:
- read file contents
- format the file in this way: blob <size>\0content
- compress the formatted file
- generate the sha-1 hash using the formatted content
- create a dir with 1st letter of hash and a file with rest of the hash
- store the compressed data in the file

     