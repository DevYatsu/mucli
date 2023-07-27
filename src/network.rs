use ifcfg;

pub fn network_command() {
    if let Ok(ifaces) = ifcfg::IfCfg::get() {
        for iface in ifaces {
            println!("Interface: {}", iface.name);
            for adress in &iface.addresses {
                if let Some(_) = adress.address {
                    println!(
                        "{:?} Address: {:?}; Mask: {:?}; Hop: {:?}",
                        adress.address_family,
                        adress.address.unwrap(),
                        adress.mask,
                        adress.hop
                    );
                } else {
                    println!(
                        "{:?} Address: {:?}; Mask: {:?}; Hop: {:?}",
                        adress.address_family, adress.address, adress.mask, adress.hop
                    );
                };
            }

            println!("MAC Address: {}", iface.mac);
            println!("\n");
        }
    } else {
        println!("Failed to retrieve network information.");
    }
}
