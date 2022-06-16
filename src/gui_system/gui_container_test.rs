pub struct WOne{
    pub index: u32,
    pub ans: u32
}

pub struct WTwo{
    pub index: u32,
    pub ans: u32
}


impl WOne{
    pub fn new()->Self{
        Self { index: 0, ans: 202 }
    }
}

impl WTwo{
    pub fn new()->Self{
        Self { index: 2, ans: 859 }
    }
}

impl GUIContainer for WOne{
    fn get_name(&self)->&str {
        "WOne"
    }

    fn handle_event(&self, event: &mut UIEvent, public_data: &mut PublicData) {
        println!("handling events WOne")
    }
}

impl GUIContainer for WTwo{
    fn get_name(&self)->&str {
        "WTwo"
    }

    fn handle_event(&self, event: &mut UIEvent, public_data: &mut PublicData) {
        println!("handling events WTwo")
    }   
}

#[test]
pub fn testing_proper_downcast(){
    let mut gui_containers = Slotmap::<Box<dyn GUIContainer>>::new_with_capacity(10);
    let gui_container = WOne::new();
    let key0 = gui_containers.push(Box::new(gui_container));
    let gui_container = WTwo::new();
    let key1 = gui_containers.push(Box::new(gui_container));

    {
        let container_thing = gui_containers.get_value_mut(&key1.expect(""));
        let c_t = container_thing.unwrap();
        let thing: &WTwo = AsAny::as_any(c_t.as_ref()).downcast_ref().expect("imposible to cast?");
    }
}