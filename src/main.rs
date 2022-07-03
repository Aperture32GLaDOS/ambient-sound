use soloud::*;
use std::env;
use std::path;
use std::collections::HashMap;
use std::io;

fn display_noise_options(noises: &Vec<&str>)
{
    let mut output: String = String::new();
    let mut counter: usize = 1;
    for noise in noises
    {
        output = output + &format!(", {}: {}", counter, noise);
        counter += 1;
    }
    output = output.split_at(1).1.to_string();
    println!("{}", output);
}

fn load_noise(index: usize, waveforms:  &mut Vec<Wav>, sounds: &mut Vec<Handle>, indices: &mut HashMap<String, usize>, noise: &str, sound: &mut Soloud, selected_volume: &f32)
{
    let mut asset: String = String::new();
    match index {
        0 => asset = "assets/main-birds.mp3".to_string(),
        1 => asset = "assets/main-crickets.mp3".to_string(),
        2 => asset = "assets/main-fire.mp3".to_string(),
        3 => asset = "assets/main-people.mp3".to_string(),
        4 => asset = "assets/main-rain.mp3".to_string(),
        5 => asset = "assets/main-sbowl.mp3".to_string(),
        6 => asset = "assets/main-thunder.mp3".to_string(),
        7 => asset = "assets/main-waves.mp3".to_string(),
        8 => asset = "assets/main-whitenoise.mp3".to_string(),
        9 => asset = "./assets/main-wind.mp3".to_string(),
        _ => {}
    }
    let mut path: path::PathBuf = env::current_exe().unwrap();
    path.pop();
    path.push(path::Path::new(&asset));
    let new_index: usize = waveforms.len();
    let mut waveform: Wav = Wav::default();
    waveform.load(path).unwrap();
    waveform.set_looping(true);
    waveforms.push(waveform);
    indices.insert(noise.to_string(), new_index);
    sounds.push(sound.play(&waveforms[new_index]));
    sound.set_volume(sounds[new_index], *selected_volume);
}


fn main() {
    let mut sound = Soloud::default().unwrap();
    let mut indices: HashMap<String, usize> = HashMap::new();
    let mut sounds: Vec<Handle> = Vec::new();
    let mut waveforms: Vec<Wav> = Vec::new();
    let mut selected_volume: f32 = 0.0;
    let mut selected_index;
    let mut buffer: String = String::new();
    let mut selected_sound: &str;
    let noises: Vec<&str> = vec!["Birds", "Crickets", "Fire", "People", "Rain", "Singing Bowl", "Thunder", "Waves", "White noise", "Wind"];
    loop
    {
        selected_volume = 0.0;
        display_noise_options(&noises);
        buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_string();
        if buffer.split(";").collect::<Vec<&str>>().len() > 1
        {
            selected_volume = buffer.split(";").collect::<Vec<&str>>()[1].to_string().trim().parse().unwrap_or(0.0);
            buffer = buffer.split(";").collect::<Vec<&str>>()[0].to_string();
        }
        selected_index = buffer.parse::<usize>().unwrap_or(10);
        if selected_index == 0 || selected_index > 10
        {
            break;
        }
        selected_index = selected_index - 1;
        selected_sound = noises[selected_index];
        if indices.get(selected_sound).is_some()
        {
            sound.set_volume(sounds[*indices.get(selected_sound).unwrap()], selected_volume);
        }
        else
        {
            load_noise(selected_index, &mut waveforms, &mut sounds, &mut indices, &mut selected_sound, &mut sound, & selected_volume);
        }
    }
}
