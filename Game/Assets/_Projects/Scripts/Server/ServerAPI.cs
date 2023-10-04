using UnityEngine;
using UnityEngine.UI;
using System.Collections;
using UnityEngine.Networking;
using System.Collections.Generic;

public class ServerAPI : MonoBehaviour
{
    [SerializeField] GameObject welcomePanel;
    [SerializeField] Text user;
    [Space]
    [SerializeField] InputField username;
    [SerializeField] InputField password;

    [SerializeField] Text errorMessages;
    [SerializeField] GameObject progressCircle;

    [SerializeField] Button loginButton;

    [SerializeField] string apiUrl; // URL API yang akan Anda panggil

    UnityWebRequest www;

    public void OnLoginButtonClicked()
    {
        loginButton.interactable = false;
        progressCircle.SetActive(true);
        StartCoroutine(Login());
    }

    IEnumerator Login()
    {
        // Buat URL lengkap dengan endpoint yang sesuai
        string endpoint = "/auth/login"; // Ganti dengan endpoint yang sesuai di API Anda
        string url = apiUrl + endpoint;

        // Buat objek JSON untuk mengirim data username dan password
        string json = "{\"user\": {\"email\": \"" + username.text + "\", \"password\": \"" + password.text + "\"}}";

        // Buat permintaan POST dengan data JSON
        www = UnityWebRequest.Post(url, "application/json");
        byte[] jsonBytes = System.Text.Encoding.UTF8.GetBytes(json);
        www.uploadHandler = new UploadHandlerRaw(jsonBytes);
        www.SetRequestHeader("Content-Type", "application/json");
        yield return www.SendWebRequest();

        if (www.result == UnityWebRequest.Result.ConnectionError || www.result == UnityWebRequest.Result.ProtocolError)
        {
            errorMessages.text = "Error: " + www.error;
            Debug.Log("<color=red>" + www.downloadHandler.text + "</color>");
        }
        else
        {
            if (www.isDone)
            {
                if (www.downloadHandler.text.Contains("error"))
                {
                    errorMessages.text = "invalid username or password!";
                    Debug.Log("<color=red>" + www.downloadHandler.text + "</color>");
                }
                else
                {
                    //open welcome panel
                    welcomePanel.SetActive(true);
                    user.text = username.text;
                    Debug.Log("<color=green>" + www.downloadHandler.text + "</color>");

                    // Panggil GetData setelah menampilkan tampilan homepage
                    GetDataMhs.Instance.GetData();
                }
            }
        }

        loginButton.interactable = true;
        progressCircle.SetActive(false);

        www.Dispose();
    }
}
