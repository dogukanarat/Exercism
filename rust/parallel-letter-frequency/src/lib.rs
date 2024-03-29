#![feature(scoped_threads)]
use std::collections::HashMap;
use std::{thread};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

type HashMapType = HashMap<char, usize>;

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
fn parFrequency(tx: Sender<HashMapType>, rx: Receiver<&[char]>) 
{
    let mut resultHashMap: HashMap<char, usize> = HashMap::new();
    let searchContent = rx.recv().unwrap();

    for (_, &currentLetter) in  searchContent.iter().enumerate()
    {
        let currentLetterLowerCase = currentLetter.to_ascii_lowercase();

        if currentLetter.is_alphabetic()
        {
            if let Some(refLetter) = resultHashMap.get_mut(&currentLetterLowerCase)
            {
                *refLetter = *refLetter + 1;
            }
            else 
            {
                resultHashMap.insert(currentLetterLowerCase, 1);
            }
        }
    }

    let _ = tx.send(resultHashMap);
}

#[allow(non_snake_case)]
pub fn frequency(input: &[&str], workerCount: usize) -> HashMap<char, usize> 
{
    // result hashmap
    let mut hashMapResult = HashMap::new();

    let mut createdThreadCount = 0;

    // result channel
    let (
        channelTxHashMap, 
        channelRxHashMap
    ) = mpsc::channel::<HashMapType>();

    let mut content: String = String::new();

    for (_, &sentence) in  input.iter().enumerate()
    {
        content.push_str(sentence);
    }

    let contentArray: Vec<_> = content.chars().collect();
    let contentSize = contentArray.len();
    let chunkSize = ( contentSize + workerCount - 1) / workerCount;

    if chunkSize != 0
    {
        let mut itContentChunks = contentArray.chunks(chunkSize);

        thread::scope(|scope| {
    
            for _ in 0..workerCount {
                if let Some(chunk) = itContentChunks.next()
                {
                    // create channel to thread
                    let (
                        channelTxPartialInput, 
                        channelRxPartionInput
                    ) = mpsc::channel::<&[char]>();
            
                    // clone receive channel from thread
                    let cloneChannelTx = channelTxHashMap.clone();
    
                    let _ = channelTxPartialInput.send(chunk);
        
                    // create thread
                    scope.spawn(|| {
                        parFrequency(cloneChannelTx, channelRxPartionInput);
                    });
    
                    createdThreadCount += 1;
                }
            }
        });
    }

    for _ in 0..createdThreadCount {
        
        if let Ok(currentHashMap)= channelRxHashMap.recv()
        {
            mergeHashMap(&mut hashMapResult, currentHashMap);
        }
    }
    
    return hashMapResult
}
