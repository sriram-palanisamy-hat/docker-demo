pub fn attach_container(net: &str, pid: i32, ip: &str) -> anyhow::Result<()> {
    // create veth
    // move peer into netns
    // attach host end to bridge
    // enter netns & assign IP
    Ok(())
}
