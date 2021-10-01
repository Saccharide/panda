use crossbeam_queue::SegQueue;
use parking_lot::RwLock;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicU32, Ordering};

pub type ChannelId = u32;
pub type ChannelCB = extern "C" fn(ChannelId, *const u8, usize);

static NEXT_CHANNEL_NUMBER: AtomicU32 = AtomicU32::new(0);

lazy_static!{
static ref CHANNELS: RwLock<HashMap<ChannelId, Channel>> =
    RwLock::new(HashMap::new());
}

struct Channel {
    name: Option<String>,
    msg_receive_cb: ChannelCB,
    message_queue: SegQueue<Vec<u8>>,
}

pub fn add_channel(p_name: Option<&str>, cb: ChannelCB) -> ChannelId {
    let mut plugins = CHANNELS.write();
    let channel_id = NEXT_CHANNEL_NUMBER.fetch_add(1, Ordering::SeqCst); 
    if plugins.insert(channel_id, Channel {
        name: p_name.map(ToString::to_string),
        msg_receive_cb: cb,
        message_queue: SegQueue::new() 
    }).is_some() {
        panic!("We've somehow added a duplicate ID");
    }
    channel_id
}

pub fn poll_plugin_message(channel_id: ChannelId) -> Option<Vec<u8>> {
    let pm = CHANNELS.read();
    if let Some(plugin) = pm.get(&channel_id){
        plugin.message_queue.pop()
    } else {
        panic!("poll_plugin_message for plugin with incorrect ID");
    }
}

pub fn publish_message_from_guest(channel_id: ChannelId, msg: Vec<u8>) {
    let pm = CHANNELS.read();
    if let Some(plugin) = pm.get(&channel_id){
        let buf_ptr = msg.as_ptr();
        (plugin.msg_receive_cb)(channel_id, buf_ptr, msg.len())
    }
}

pub fn publish_message_to_guest(channel_id: ChannelId, msg: Vec<u8>) {
    let pm = CHANNELS.read();
    if let Some(plugin) = pm.get(&channel_id){
        plugin.message_queue.push(msg)
    }
}

pub fn get_channel_from_name(p_name: &str) -> Option<ChannelId>{
    CHANNELS.read().iter().find(|(_, v)| v.name.as_deref() == Some(p_name)).map(|x| *x.0)
}