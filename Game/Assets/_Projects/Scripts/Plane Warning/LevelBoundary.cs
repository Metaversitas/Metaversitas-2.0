using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using TMPro;
using Fusion;

public class LevelBoundary : MonoBehaviour
{
    // Start is called before the first frame update
    [SerializeField] private TMP_Text _levelWarning;
    public GameObject warningCanvas;
    [SerializeField] private App _app;
    string _level;


    void Start()
    {
        _app = App.FindInstance();
        SessionProps props = _app.Session.Props;
        _app.Session.LoadMap(props.StartMap);
        Session s = _app.Session;

        _level = $"{s.Props.RoomPertemuan}";
        
        // Nonaktifkan pesan peringatan pada awal permainan
        warningCanvas.SetActive(false);
    }

    private void OnTriggerEnter(Collider other)
    {
        if (other.CompareTag("Player"))
        {
            switch (_level)
            {
                case "Pertemuan 1":
                    // Setactive game objects pada lantai 1
                    _levelWarning.SetText("Pertemuan 2");
                    break;
                case "Pertemuan 2":
                    // Setactive game objects pada lantai 2 dan lantai 1 tetap aktif
                    _levelWarning.SetText("Pertemuan 3");
                    break;
                case "Pertemuan 3":
                    // Setactive game objects pada lantai 2 dan lantai 1 tetap aktif
                    _levelWarning.SetText("Pertemuan 4");
                    break;
                case "Pertemuan 4":
                    _levelWarning.SetText("Pertemuan 5");
                    break;
                // Tambahkan case-case lain sesuai kebutuhan
                default:
                    // Default case jika _level tidak sesuai dengan yang diharapkan
                    break;
            }
            warningCanvas.SetActive(true);
        }
    }
    private void OnTriggerExit(Collider other)
    {
        if (other.CompareTag("Player"))
        {
            warningCanvas.SetActive(false);
        }
    }
}
