use crate::object;

pub fn run(message: &str, tree_hash:&str, parent_hash:Option<&str>) -> anyhow::Result<()> {

    let hash = object::create_commit(tree_hash, parent_hash, message)?;
    println!("{}", hex::encode(hash));

    Ok(())
    
}

/*
commit <size>\0
tree    tree_sha(20bytes)
author   codebyred <nazmulhaqueredoan@gmail.com> 1748355565 +0600
committer codebyred <nazmulhaqueredoan@gmail.com> 1748355565 +0600
*/