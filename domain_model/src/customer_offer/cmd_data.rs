use super::cmds::*;

pub enum CmdData {
    AddItem(AddItem), 
    RemoveItem(RemoveItem)
}