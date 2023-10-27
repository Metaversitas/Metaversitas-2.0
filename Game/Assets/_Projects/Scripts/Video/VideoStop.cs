using UnityEngine;
using UnityEngine.Video;
using System;

public class VideoStop : MonoBehaviour
{
    [SerializeField] PlayerStateManager _playerStateManager;
    public VideoPlayer videoPlayer;

    void Start()
    {
        if (videoPlayer != null)
        {
            _playerStateManager.TriggerIntroState();
            Debug.Log(_playerStateManager.CurrentGameState);
        }

        if (videoPlayer == null)
        {
            Debug.LogError("VideoPlayer reference not set. Please assign the VideoPlayer component.");
            return;
        }

        // Mengaitkan metode OnVideoEnd ke peristiwa ketika video selesai
        videoPlayer.loopPointReached += OnVideoEnd;

        // Menonaktifkan kemampuan pause selama video sedang diputar
        _playerStateManager.PlayState += OnPlayState;
        _playerStateManager.PauseState += OnPauseState;
    }

    void Update()
    {
        // Mencegah pemain menekan tombol Escape selama video berlangsung
        if (_playerStateManager.IsVideoPlaying && Input.GetKeyDown(KeyCode.Escape))
        {
            Debug.Log("Escape key is disabled during video playback.");
            return;
        }
    }

    void OnPlayState(object sender, EventArgs e)
    {
        // Pastikan videoPlayer tidak null dan belum dihancurkan sebelum mengaksesnya
        if (videoPlayer != null && !videoPlayer.Equals(null))
        {
            // Nonaktifkan kemampuan pause selama video sedang diputar
            videoPlayer.playbackSpeed = 1.0f;
        }
    }

    void OnPauseState(object sender, EventArgs e)
    {
        // Pastikan videoPlayer tidak null dan belum dihancurkan sebelum mengaksesnya
        if (videoPlayer != null && !videoPlayer.Equals(null))
        {
            // Kembalikan kecepatan pemutaran video ke 0 untuk menonaktifkan pause
            videoPlayer.playbackSpeed = 0.0f;
        }
    }

    void OnVideoEnd(VideoPlayer vp)
    {
        // Pastikan videoPlayer tidak null dan belum dihancurkan sebelum mengaksesnya
        if (videoPlayer != null && !videoPlayer.Equals(null))
        {
            // Video telah selesai, lakukan tindakan yang diinginkan di sini
            // Misalnya, nonaktifkan objek yang berisi pemutar video
            gameObject.SetActive(false);

            _playerStateManager.TriggerPlayState();
        }
    }
}



