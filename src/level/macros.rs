macro_rules! set {
	($($item:expr)*) => {
		{
			let mut set = std::collections::HashSet::new();
			$(set.insert($item);)*
			set
		}
	}
}

macro_rules! fail_set {
    ($change_set:expr) => {
        $change_set
            .iter()
            .filter_map(|(entity, _)| match entity {
                Entity::Block(index) => Some(*index),
                Entity::Player(_) => None,
            })
            .collect()
    };
}

macro_rules! entity_fail {
    ($item:expr) => {
        match $item {
            Some(index) => set!(index),
            None => std::collections::HashSet::new(),
        }
    };
}
