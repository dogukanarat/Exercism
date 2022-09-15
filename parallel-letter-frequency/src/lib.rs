

use std::collections::HashMap;
use std::{thread, string};
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

type HashMapType = HashMap<char, usize>;
type ThreadType = JoinHandle<()>;

#[allow(non_snake_case)]
fn mergeHashMap(destination: &mut HashMapType, source: HashMapType) 
{
    for (key, value) in destination.iter_mut()
    {
        if let Some(sourceValue) = source.get(key)
        {
            *value += sourceValue;
        }
    }

    for (key, value) in source.iter()
    {
        if destination.get(key).is_none()
        {
            destination.insert(*key, *value);
        }
    }
}

#[allow(non_snake_case)]
fn parFrequency(_tx: Sender<HashMapType>, _rx: Receiver<&[&str]>) 
{
    let mut resultHashMap = HashMap::new();

    resultHashMap.insert('a', 11);

    let _result = _tx.send(resultHashMap);
}

#[allow(non_snake_case)]
pub fn frequency(input: &[&str], workerCount: usize) -> HashMap<char, usize> {

    // result hashmap
    let mut hashMapResult = HashMap::new();

    // result channel
    let (
        channelTxHashMap, 
        channelRxHashMap
    ) = mpsc::channel::<HashMapType>();
    
    // thread collection
    let mut threads: Vec<ThreadType> = Vec::with_capacity(workerCount);

    for _ in 0..workerCount {
        // create channel to thread
        let (
            channelTxPartialInput, 
            channelRxPartionInput
        ) = mpsc::channel::<&[&str]>();

        // clone receive channel from thread
        let cloneChannelTx = channelTxHashMap.clone();

        let mut iterInputChunks = input.chunks(2);
        
        if let Some(chunk) = iterInputChunks.next()
        {
            let _resultSend = channelTxPartialInput.send(chunk);

            // create thread
            let currentThread = thread::spawn(|| {
                parFrequency(cloneChannelTx, channelRxPartionInput);
            });

            // push created thread to vector
            threads.push(currentThread);
        }
    }

    while threads.len() > 0 {
        let currenThread = threads.remove(0);
        currenThread.join().unwrap();
    }

    for _ in 0..workerCount {
        
        if let Ok(currentHashMap)= channelRxHashMap.recv()
        {
            mergeHashMap(&mut hashMapResult, currentHashMap);
        }
    }
    
    return hashMapResult
}
