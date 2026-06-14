use crate::core::settings_manager::settings_manager::SettingsManager;

pub struct AudioManager {
    volume: u8,
}

impl AudioManager {
    // Constructeur qui utilise une valeur par défaut pour le volume.
    pub fn new() -> Self {
        AudioManager::default()
    }

    // Constructeur avec volume spécifique.
    pub fn with_volume(volume_initial: u8) -> Self {
        AudioManager {
            volume: volume_initial.clamp(0, 100), // S'assurer que le volume est dans la plage valide.
        }
    }

    // Setter pour le volume.
    pub fn set_volume(&mut self, volume: u8) {
        self.volume = volume.clamp(0, 100); // Applique le clamp ici aussi pour la sécurité.
        println!("Volume set to: {}", self.volume);
    }

    pub fn get_volume(&self) -> u8 {
        self.volume
    }

    // Une fonction associée pour obtenir une instance avec la valeur par défaut.
    pub fn default() -> Self {
        let mut volume = 50;

        //tentative de récupération de la valeur saved
        if let Ok(config) = SettingsManager::load_config() {
            volume = config.volume_initial;
        }

        AudioManager {
            volume: volume, // Valeur par défaut.
        }
    }
}
