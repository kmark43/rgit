pub fn find_object_path(hash: &str) -> String {
    let path = format!(".git/objects/{}/{}", &hash[0..2], &hash[2..]);

    path
}
