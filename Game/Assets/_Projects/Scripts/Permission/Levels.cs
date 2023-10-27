using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Video;

public class Levels : MonoBehaviour
{
    [SerializeField] private List<GameObject> gameObjects;
    [SerializeField] private List<VideoClip> introVideos;
    [SerializeField] private VideoPlayer introVideoPlayer;

    [SerializeField] private App _app;
    private string _level;
    private string _gamemode;

    void Start()
    {
        InitializeSession();

        DisplayIntroVideo();

        switch (_gamemode)
        {
            case "LabFisika":
                // Set active game objects on floor 1
                break;
            case "Candi":
                // Set active game objects on floor 2 while keeping floor 1 active
                SetActiveGameObjects();
                break;
            case "Museum":
                // Set active game objects on floor 2 while keeping floor 1 active
                break;
            case "Malioboro":
                // Add code for Malioboro
                break;
            // Add other cases as needed
            default:
                // Default case if _level is not as expected
                break;
        }

        Debug.Log(_gamemode);
        Debug.Log(_level);
    }

    void InitializeSession()
    {
        _app = App.FindInstance();
        SessionProps props = _app.Session.Props;
        _app.Session.LoadMap(props.StartMap);
        Session s = _app.Session;

        _level = $"{s.Props.RoomPertemuan}";
        _gamemode = $"{s.Props.PlayMode}";
    }

    void DisplayIntroVideo()
    {
        int levelIndex = int.Parse(_level.Replace("Pertemuan ", "")) - 1;

        if (levelIndex < introVideos.Count)
        {
            introVideoPlayer.clip = introVideos[levelIndex];
            introVideoPlayer.Play();
            Debug.Log($"Introductory video for {_level} is playing.");
        }
        else
        {
            Debug.LogError($"No intro video found for {_level}.");
        }
    }

    void SetActiveGameObjects()
    {
        switch (_level)
        {
            case "Pertemuan 2":
                SetFloor2Active();
                break;
            case "Pertemuan 3":
                SetFloor3Active();
                break;
            case "Pertemuan 4":
                SetFloor4Active();
                break;
            case "Pertemuan 5":
                SetFloor5Active();
                break;
            // Add other cases as needed
            default:
                // Default case if _level is not as expected
                break;
        }
    }

    void SetFloor2Active()
    {
        SetFloorActive(4);
    }

    void SetFloor3Active()
    {
        SetFloorActive(8);
    }

    void SetFloor4Active()
    {
        SetFloorActive(12);
    }

    void SetFloor5Active()
    {
        SetFloorActive(16);
    }

    void SetFloorActive(int objectCount)
    {
        int cnt = 0;
        foreach (GameObject obj in gameObjects)
        {
            if (cnt < objectCount)
            {
                obj.SetActive(false);
            }
            else
            {
                break;
            }
            cnt++;
        }
    }
}
