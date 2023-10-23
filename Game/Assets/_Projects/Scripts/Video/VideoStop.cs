using UnityEngine;
using UnityEngine.Video;

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
    }

    void OnVideoEnd(VideoPlayer vp)
    {
        // Video telah selesai, lakukan tindakan yang diinginkan di sini
        // Misalnya, nonaktifkan objek yang berisi pemutar video
        gameObject.SetActive(false);

        _playerStateManager.TriggerPlayState();
    }
}
