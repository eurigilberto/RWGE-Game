
struct TestData{
    pub size: UVec2,
    pub collection: Vec<f32>
}

impl TestData {
    pub fn new()->Self{
        Self { size: uvec2(0, 50), collection: Vec::new() }
    }
}

#[test]
fn create_and_store_anymap() {
    let mut rt_slotmap = Slotmap::<RenderTexture>::new_with_capacity(10);
    let mut td_slotmap = Slotmap::<TestData>::new_with_capacity(20);

    let mut td = TestData::new();
    td.size.x = 20;

    let td_key = td_slotmap.push(td).expect("Could not add data to slotmap");

    let mut anymap = anymap::Anymap::new();

    anymap.insert(rt_slotmap);
    anymap.insert(td_slotmap);

    {
        let sl = anymap.get::<Slotmap<TestData>>().expect("Test data slotmap was not added");
        let td_1 = sl.get_value(&td_key).expect("Could not get recently added value");
        assert_eq!(td_1.size.x, 20);
    }

    {
        let sl = anymap.get_mut::<Slotmap<TestData>>().expect("Test data slotmap was not added");
        let td_1 = sl.get_value_mut(&td_key).expect("Could not get recently added value");
        td_1.size.y = 46541;
        assert_eq!(td_1.size.x, 20);
    }

    {
        let sl = anymap.get::<Slotmap<TestData>>().expect("Test data slotmap was not added");
        let td_1 = sl.get_value(&td_key).expect("Could not get recently added value");
        assert_eq!(td_1.size.x, 20);
        assert_eq!(td_1.size.y, 46541);
    }
}