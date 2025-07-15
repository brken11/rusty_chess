
#[derive(Debug, Copy, Clone)]
pub enum ThreadIdentifier{
    Main(ThreadIdHash),
    Log(ThreadIdHash),
    Network(ThreadIdHash),
    Game(ThreadIdHash),
    UI(ThreadIdHash),
    GUI(ThreadIdHash),
    Other(ThreadIdHash,&'static str),
}

impl ThreadIdentifier {
    pub(crate) fn generate_id() -> ThreadIdHash {
        ThreadIdHash::new()
    }
}

pub type ThreadIdHash = u128;
pub trait ThreadIdHashExt{
    fn new() -> Self;
}
impl ThreadIdHashExt for ThreadIdHash {
    fn new() -> Self {
        std::time::Instant::now().elapsed().as_nanos()
    }
}

impl PartialEq for ThreadIdentifier {
    fn eq(&self, other: &Self) -> bool {
        match &self {
            ThreadIdentifier::Main(id) => match &other {ThreadIdentifier::Main(other_id) => id == other_id, _ => false},
            ThreadIdentifier::Log(id) => match &other {ThreadIdentifier::Log(other_id) => id == other_id, _ => false},
            ThreadIdentifier::Network(id) => match &other {ThreadIdentifier::Network(other_id) => id == other_id, _ => false},
            ThreadIdentifier::Game(id) => match &other {ThreadIdentifier::Game(other_id) => id == other_id, _ => false},
            ThreadIdentifier::UI(id) => match &other {ThreadIdentifier::UI(other_id) => id == other_id, _ => false},
            ThreadIdentifier::GUI(id) => match &other {ThreadIdentifier::GUI(other_id) => id == other_id, _ => false},
            ThreadIdentifier::Other(id, name) => match &other {ThreadIdentifier::Other(other_id, other_name) => id == other_id && name == other_name, _ => false},
        }
    }
}


impl std::fmt::Display for ThreadIdentifier{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ThreadIdentifier::Other(id,name) => write!(f,"Other[{}:{}]",name,id),
            ThreadIdentifier::Main(_) => write!(f,"Main"),
            ThreadIdentifier::Log(_) => write!(f,"Log"),
            ThreadIdentifier::Network(_) => write!(f,"Network"),
            ThreadIdentifier::Game(_) => write!(f,"Game"),
            ThreadIdentifier::UI(_) => write!(f,"UI"),
            ThreadIdentifier::GUI(_) => write!(f,"GUI"),
        }
    }
}

