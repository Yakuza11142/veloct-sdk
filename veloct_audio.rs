// ============================================================================
// VELOCT SPATIAL ENGINE - NATIVE FORMANT VOICE SYNTHESIZER
// Extends: veloct_audio.rs
// Zero-Dependency Formant & Pitch Modulator for Any Voice Type
// ============================================================================

#[repr(C, align(16))]
pub struct VoiceTypeVector {
    pub pitch_f0: f32,       // Base pitch (e.g., 85Hz = Deep Male, 220Hz = Female, 350Hz = Child)
    pub formant_f1: f32,     // Throat Resonance
    pub formant_f2: f32,     // Vocal Cavity Resonance
    pub roughness: f32,      // Distortion/Grit (0.0 = Smooth, 1.0 = Monster/Robot)
}

impl VeloctAudioEngine {
    // Generates and modulates any voice type directly into raw hardware audio buffers
    #[inline(always)]
    pub unsafe fn synthesize_voice_frame(
        voice: &VoiceTypeVector,
        phase: &mut f32,
        buffer: &mut [f32; 512]
    ) {
        let sample_rate = 48000.0;
        let phase_increment = (2.0 * std::f32::consts::PI * voice.pitch_f0) / sample_rate;

        for i in 0..512 {
            *phase += phase_increment;
            if *phase > 2.0 * std::f32::consts::PI {
                *phase -= 2.0 * std::f32::consts::PI;
            }

            // Fundamental carrier wave
            let fundamental = phase.sin();

            // Formant Bandpass Filters (Shapes pitch into specific vocal timbre)
            let f1_band = (*phase * (voice.formant_f1 / voice.pitch_f0)).sin() * 0.6;
            let f2_band = (*phase * (voice.formant_f2 / voice.pitch_f0)).sin() * 0.3;

            // Roughness / Distortion Modulation (For alien, mechanical, or monster voice types)
            let noise = if voice.roughness > 0.0 {
                (phase.sin() * 100.0).sin() * voice.roughness
            } else {
                0.0
            };

            // Raw PCM output sample combining pitch, formants, and texture
            buffer[i] = (fundamental + f1_band + f2_band + noise).clamp(-1.0, 1.0);
        }
    }
}
