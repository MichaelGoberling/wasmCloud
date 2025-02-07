const DEFAULT_TOPIC_PREFIX: &str = "wasmbus.ctl";

fn prefix(topic_prefix: &Option<String>, lattice: &str) -> String {
    format!(
        "{}.{}",
        topic_prefix
            .as_ref()
            .unwrap_or(&DEFAULT_TOPIC_PREFIX.to_string()),
        lattice
    )
}

pub fn provider_auction_subject(topic_prefix: &Option<String>, lattice: &str) -> String {
    format!("{}.provider.auction", prefix(topic_prefix, lattice))
}

pub fn actor_auction_subject(topic_prefix: &Option<String>, lattice: &str) -> String {
    format!("{}.actor.auction", prefix(topic_prefix, lattice))
}

pub fn advertise_link(topic_prefix: &Option<String>, lattice: &str) -> String {
    format!("{}.link.put", prefix(topic_prefix, lattice))
}

pub fn remove_link(topic_prefix: &Option<String>, lattice: &str) -> String {
    format!("{}.link.del", prefix(topic_prefix, lattice))
}

pub fn publish_registries(topic_prefix: &Option<String>, lattice: &str) -> String {
    format!("{}.registry.put", prefix(topic_prefix, lattice))
}

pub fn put_config(topic_prefix: &Option<String>, lattice: &str, config_name: &str) -> String {
    format!("{}.config.put.{config_name}", prefix(topic_prefix, lattice))
}

pub fn delete_config(topic_prefix: &Option<String>, lattice: &str, config_name: &str) -> String {
    format!("{}.config.del.{config_name}", prefix(topic_prefix, lattice))
}

pub fn put_label(topic_prefix: &Option<String>, lattice: &str, host_id: &str) -> String {
    format!("{}.label.put.{host_id}", prefix(topic_prefix, lattice))
}

pub fn delete_label(topic_prefix: &Option<String>, lattice: &str, host_id: &str) -> String {
    format!("{}.label.del.{host_id}", prefix(topic_prefix, lattice))
}

pub mod commands {
    use super::prefix;

    pub fn scale_actor(topic_prefix: &Option<String>, lattice: &str, host_id: &str) -> String {
        format!("{}.actor.scale.{host_id}", prefix(topic_prefix, lattice))
    }

    pub fn start_provider(topic_prefix: &Option<String>, lattice: &str, host_id: &str) -> String {
        format!("{}.provider.start.{host_id}", prefix(topic_prefix, lattice))
    }

    pub fn stop_provider(topic_prefix: &Option<String>, lattice: &str, host_id: &str) -> String {
        format!("{}.provider.stop.{host_id}", prefix(topic_prefix, lattice))
    }

    pub fn update_actor(topic_prefix: &Option<String>, lattice: &str, host_id: &str) -> String {
        format!("{}.actor.update.{host_id}", prefix(topic_prefix, lattice))
    }

    pub fn stop_host(topic_prefix: &Option<String>, lattice: &str, host_id: &str) -> String {
        format!("{}.host.stop.{host_id}", prefix(topic_prefix, lattice))
    }
}

pub mod queries {
    use super::prefix;

    pub fn link_definitions(topic_prefix: &Option<String>, lattice: &str) -> String {
        format!("{}.link.get", prefix(topic_prefix, lattice))
    }

    pub fn claims(topic_prefix: &Option<String>, lattice: &str) -> String {
        format!("{}.claims.get", prefix(topic_prefix, lattice))
    }

    pub fn host_inventory(topic_prefix: &Option<String>, lattice: &str, host_id: &str) -> String {
        format!("{}.host.get.{host_id}", prefix(topic_prefix, lattice))
    }

    pub fn hosts(topic_prefix: &Option<String>, lattice: &str) -> String {
        format!("{}.host.ping", prefix(topic_prefix, lattice))
    }

    pub fn config(topic_prefix: &Option<String>, lattice: &str, config_name: &str) -> String {
        format!("{}.config.get.{config_name}", prefix(topic_prefix, lattice),)
    }
}
