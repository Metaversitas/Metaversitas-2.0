using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Levels : MonoBehaviour
{

    [SerializeField] public  List<GameObject> gameObjects;


    [SerializeField] private App _app;
    string _level;
    string _gamemode;


    void Start()
    {
        _app = App.FindInstance();
        SessionProps props = _app.Session.Props;
        _app.Session.LoadMap(props.StartMap);
        Session s = _app.Session;

        _level = $"{s.Props.RoomPertemuan}";
        _gamemode = $"{s.Props.PlayMode}";

        switch (_gamemode)
        {
            case "LabFisika":
                // Setactive game objects pada lantai 1
                
                break;
            case "Candi":
                // Setactive game objects pada lantai 2 dan lantai 1 tetap aktif
                SetActiveGameObjects();
                break;
            case "Museum":
                // Setactive game objects pada lantai 2 dan lantai 1 tetap aktif
                
                break;
            case "Malioboro":
                
                break;
            // Tambahkan case-case lain sesuai kebutuhan
            default:
                // Default case jika _level tidak sesuai dengan yang diharapkan
                break;
        }
        Debug.LogError(_gamemode);
        Debug.LogError(_level);
    }

    void SetActiveGameObjects()
    {
        switch (_level)
        {
            case "Pertemuan 2":
                // Setactive game objects pada lantai 1
                SetLantai2Active();
                break;
            case "Pertemuan 3":
                // Setactive game objects pada lantai 2 dan lantai 1 tetap aktif
                SetLantai3Active();
                break;
            case "Pertemuan 4":
                // Setactive game objects pada lantai 2 dan lantai 1 tetap aktif
                SetLantai4Active();
                break;
            case "Pertemuan 5":
                SetLantai5Active();
                break;
            // Tambahkan case-case lain sesuai kebutuhan
            default:
                // Default case jika _level tidak sesuai dengan yang diharapkan
                break;
        }
    }

    void SetLantai2Active()
    {
        // Setactive game objects pada lantai 1
        int cnt = 0;
        foreach (GameObject obj in gameObjects)
        {
            if (cnt < 4)
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

    void SetLantai3Active()
    {
        // Setactive game objects pada lantai 2
        // Untuk setactive lantai 1, pastikan mereka sudah aktif sebelum memanggil method ini
        int cnt = 0;
        foreach (GameObject obj in gameObjects)
        {
            if (cnt < 8)
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

    void SetLantai4Active()
    {
        // Setactive game objects pada lantai 2
        // Untuk setactive lantai 1, pastikan mereka sudah aktif sebelum memanggil method ini
        int cnt = 0;
        foreach (GameObject obj in gameObjects)
        {
            if (cnt < 12)
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

    void SetLantai5Active()
    {
        // Setactive game objects pada lantai 2
        // Untuk setactive lantai 1, pastikan mereka sudah aktif sebelum memanggil method ini
        int cnt = 0;
        foreach (GameObject obj in gameObjects)
        {
            if (cnt < 16)
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


    // Update is called once per frame
}
