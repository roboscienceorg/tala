extern crate serde_json;
extern crate serde;
extern crate serde_derive;

use std::collections::HashMap;
use pyo3::prelude::*;
#[path = "messaging.rs"] mod messaging;


type IPPort = (String, u16);        //tuple that holds (IP, Port)


/**
 * Publishers send data to channels
 * channelInfo (HashMap<String, IPPort>) - Channel names maped to their addresses
 * masterip (String) - IP of Mster Process
 * masterport (u16) - port of Master Process
 * ip (String) - Publisher's IP address
 * port (u16) - Publisher's port
 */
#[pyclass]
#[derive(Clone)]
pub struct Publisher
{
    pub channelInfo : HashMap<String,IPPort>,
    pub masterip: String,
    pub masterport: u16,
    pub ip: String,
    pub port:   u16,

}

impl Publisher
{
    /**
     * constructor for Publisher Object
     * MasterIP (String) - Ip of Master Process
     * MasterPort (u16) - Port of Master Process
     * IP (String) - IP to host publisher on
     * Port (u16) - Port to host publisher on
     */
    pub fn new(MasterIP: String, MasterPort: u16, IP: String, Port: u16) -> Publisher
    {
        return Publisher{channelInfo: HashMap::new(), masterip: MasterIP, masterport: MasterPort, ip : IP, port : Port}
    }

    //fn for adding a channel info to the map being used for data storage
    fn add(&mut self, Name: String, IP: String, Port: u16)
    {
        self.channelInfo.insert(Name, (IP, Port));   //pass in the info about the channel to be stored in the pub
    }
    //function for connecting to a channel

}

#[pymethods]
impl Publisher{

    /**
     * Overrides the tostring representation fro publisher
     */
    pub fn to_string(&mut self) -> String
    {
        return format!("Construct Pub: Master({}, {}) Self({}, {})", self.masterip, self.masterport, self.ip, self.port);
    }

    /**
     * Connets a Publisher to a Channel
     * Name (String) - Channel name to connect to
     */
    pub fn connect(&mut self, Name: String)
    {
        //if it is not stored in the list open up a req socket and send a request to master asking for channel info
        if  self.channelInfo.contains_key(&Name) == false
        {
            let message_ = messaging::Message { messageType: 'c', ip: self.ip.to_string(), port: self.port,  message: Name.to_string() };
            let m2 = messaging::send(self.masterip.to_string(), self.masterport, message_);
            //add the information to the channelInfo Object
            self.add(Name, m2.ip, m2.port);
        }
    }
    
    /**
     * Disconnects a publisher from a Channel
     * Name (String) - Channel to disconnect from
     */
    pub fn disconnect(&mut self, Name: String)
    {
        //Check if channel is stored in hashmap
        if  self.channelInfo.contains_key(&Name)
        {
            let mx = messaging::Message {messageType: 'd',ip: self.ip.to_string(),port: self.port,message: Name.to_string()};
            let _ = messaging::send(self.masterip.to_string(), self.masterport, mx);
            self.channelInfo.remove(&Name);
        }
        else    //If the channel does not exist in the publisher then don't do anything
        {
        }
    }

    /**
     * Sends data to a channel
     * ChannelName (String) - Name of channel to send to
     * Mess (String) - Message to send
     */
    pub fn publish(&mut self, ChannelName : String, Mess: String)
    {
        

        if  self.channelInfo.contains_key(&ChannelName) == false
        {

            self.connect(ChannelName.to_string());
        }


        let chanInfo = self.channelInfo.get(&ChannelName).unwrap();
        let mx = messaging::Message { messageType: 'D', ip: self.ip.to_string(), port: self.port,  message: Mess.to_string() };

        messaging::send(chanInfo.0.to_string(), chanInfo.1, mx);


    }

    /**
     * Returns IP of Publisher
     */
    pub  fn getIP(&mut self)->String
    {
        return self.ip.to_string();
    }

    /**
     * Returns port of publisher
     */
    pub  fn getPort(&mut self)->u16
    {
        return self.port;
    }
}
