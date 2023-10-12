using UnityEngine;
using UnityEngine.UI;
using System.Collections;
using UnityEngine.Networking;
using Fusion.Photon.Realtime;
using System.Collections.Generic;
using UnityEngine.SceneManagement;

public class ServerAPI : MonoBehaviour
{
    [SerializeField] GameObject welcomePanel;
    [SerializeField] Text user;
    [Space]
    [SerializeField] InputField username;
    [SerializeField] InputField passwords;

    [SerializeField] Text errorMessages;
    [SerializeField] GameObject progressCircle;

    [SerializeField] Button loginButton;

    [SerializeField] string apiUrl;

    UnityWebRequest www;

    private App _app;

    void Start()
    {
        _app = App.FindInstance();

        if (_app == null)
        {
            Debug.LogError("Instance of App not found!");
        }
    }

    public void OnLoginButtonClicked()
    {
        loginButton.interactable = false;
        progressCircle.SetActive(true);
        StartCoroutine(Login());
    }

    IEnumerator Login()
    {
        string endpoint = "/auth/login";
        string url = apiUrl + endpoint;

        string json = "{\"user\": {\"email\": \"" + username.text + "\", \"password\": \"" + passwords.text + "\"}}";

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
                    var response_headers = www.GetResponseHeaders();
                    foreach (var key in response_headers.Keys)
                    {
                        Debug.Log("Headers: \n");
                        Debug.Log($"{key}:{response_headers[key]}");
                    }
                    //SetPhotonAuthentication();

                    // Panggil GetData setelah menampilkan tampilan homepage
                    GetDataMhs.Instance.GetData();

                    SceneManager.LoadScene("HomeMenu");

                    loginButton.interactable = true;
                    progressCircle.SetActive(false);
                }
            }
        }

        www.Dispose();
    }

    void SetPhotonAuthentication()
    {
        var sessionProps = _app.AutoSession;
        var email = username.text;
        var password = passwords.text;

        var authValues = new AuthenticationValues
        {
            AuthType = CustomAuthenticationType.Custom
        };
        Dictionary<string, object> body = new Dictionary<string, object>
        {
            ["auth_data"] = new Dictionary<string, string>
            {
                {"cookie_auth", email},
                {"cookie_session", password}
            }
        };
        authValues.SetAuthPostData(body);

        _app.SetAuthenticationValues(authValues);
        _app.InitiateGame();
    }
}
