use std::process::Command;

pub fn create_bridge(name: &str, _subnet: &str, gateway: &str) -> anyhow::Result<()> {
    Command::new("ip").args(["link", "add", name, "type", "bridge"]).status().ok();
    Command::new("ip").args(["addr", "add", &format!("{}/24", gateway), "dev", name]).status().ok();
    Command::new("ip").args(["link", "set", name, "up"]).status()?;
    Ok(())
}

pub fn attach_container(net: &str, pid: u32, ip: &str) -> anyhow::Result<()> {
    let veth_host = format!("vethh{}", pid);
    let veth_cont = format!("vethc{}", pid);

    // create veth pair
    Command::new("ip").args([
        "link", "add", &veth_host,
        "type", "veth",
        "peer", "name", &veth_cont,
    ]).status()?;

    // move container end into netns
    Command::new("ip").args([
        "link", "set", &veth_cont,
        "netns", &pid.to_string(),
    ]).status()?;

    // attach host end to bridge
    Command::new("ip").args(["link", "set", &veth_host, "master", net]).status()?;
    Command::new("ip").args(["link", "set", &veth_host, "up"]).status()?;

    // configure container side
    Command::new("nsenter").args([
        "-t", &pid.to_string(), "-n",
        "sh", "-c",
        &format!(
            "ip link set lo up && \
             ip addr add {}/24 dev {} && \
             ip link set {} up && \
             ip route add default via 10.200.1.1",
            ip, veth_cont, veth_cont
        ),
    ]).status()?;

    Ok(())
}
