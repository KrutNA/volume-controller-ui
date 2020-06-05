use pulse::context::introspect;

const APPLICATION_NAME: &'static str = "application.name";

#[cfg(feature = "another_updater")]
#[derive(Clone, Debug, Default)]
pub struct Counter {
    pub value: usize,
}

#[cfg(feature = "another_updater")]
impl Counter {
    pub fn inc(&mut self) {
        self.value += 1;
    }
}

#[derive(Clone, Debug, Default)]
pub struct SinkInputData {
    pub id: u32,
    pub name: String,
    pub volume: u32,
    pub mute: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MainData {
    pub volume: u32,
    pub mute: bool,
}

impl<'a> From<&'a introspect::SinkInputInfo<'a>> for SinkInputData {
    fn from(item: &'a introspect::SinkInputInfo<'a>) -> Self {
        Self {
            id: item.index.clone(),
            name: item.proplist.get_str(APPLICATION_NAME).unwrap(),
            volume: item.volume.get()[0].0.clone(),
            mute: item.mute.clone(),
        }
    }
}

#[allow(dead_code)]
impl<'a> From<&'a introspect::SinkInfo<'a>> for MainData {
    fn from(item: &'a introspect::SinkInfo<'a>) -> Self {
        Self {
            volume: item.volume.get()[0].0.clone(),
            mute: item.mute.clone(),
        }
    }
}

#[allow(dead_code)]
impl<'a> From<&'a introspect::SourceInfo<'a>> for MainData {
    fn from(item: &'a introspect::SourceInfo<'a>) -> Self {
        Self {
            volume: item.volume.get()[0].0.clone(),
            mute: item.mute.clone(),
        }
    }
}
