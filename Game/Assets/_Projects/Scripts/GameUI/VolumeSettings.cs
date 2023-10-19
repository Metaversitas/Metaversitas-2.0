using UnityEngine;
using UnityEngine.Audio;
using UnityEngine.UI;
public class VolumeSettings : MonoBehaviour
{
    [SerializeField] private AudioMixer myMixer;
    [SerializeField] private AudioMixer VoiceChatMixer;
    [SerializeField] private Slider musicSlider;
    [SerializeField] private Slider VoicechatSlider;

    private void Start()
    {
        if (PlayerPrefs.HasKey("musicVolume") && PlayerPrefs.HasKey("VoicechatVolume"))
        {
            LoadVolume();
        }
        else
        {
            SetMusicVolume();
            SetVoicechatVolume();
        }
    
    }

    public void SetMusicVolume()
    {
        float volume = musicSlider.value;
        myMixer.SetFloat("_sfx", Mathf.Log10(volume)*20);
        PlayerPrefs.SetFloat("musicVolume", volume);
    }

    public void SetVoicechatVolume()
    {
        float volume = VoicechatSlider.value;
        VoiceChatMixer.SetFloat("_voiceChat", Mathf.Log10(volume)*20);
        PlayerPrefs.SetFloat("VoicechatVolume", volume);
    }

    private void LoadVolume()
    {
        musicSlider.value=PlayerPrefs.GetFloat("musicVolume");
        VoicechatSlider.value=PlayerPrefs.GetFloat("VoicechatVolume");

        SetMusicVolume();
        SetVoicechatVolume();
    }
}
