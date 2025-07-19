use std::{io::{Write}, sync::{Arc, RwLock}, thread, time::Duration};


use rcli_loader::loading_element::LoadingElement;
use reqwest::Client;
use tokio::task::JoinHandle;




pub fn sim_download(loading_element: Arc<RwLock<LoadingElement>>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let url = "https://download.blender.org/demo/movies/BBB/bbb_sunflower_1080p_60fps_normal.mp4.zip"; // We <3 Big Buck Bunny... and their upload speeds ;) - (355019001 bytes - 355.02 megabytes)

        let client = Client::new();
        let mut response = match client.get(url).send().await {
            Ok(res) => res,
            Err(error) => { print!("Error creating request"); return } // TODO: Do a proper callback here. Perhaps make loader able to display ERROR instead of loading bar if not cleared?
        };
        

        let total_size = response
            .content_length()
            .unwrap_or(0);
        loading_element.write().unwrap().set_max(total_size as usize);

        println!("Total size: {} bytes", total_size);

        let mut file = std::fs::File::create("target/output.zip").unwrap();
        let mut downloaded: u64 = 0;

    
        while let Some(chunk) = response.chunk().await.unwrap() {
            file.write_all(&chunk).unwrap();
            loading_element.write().unwrap().update(chunk.len());
        }

        // println!("Download complete!"); // TODO: This will not work right now, as i need to implement a custom print function.
    })

}