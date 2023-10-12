using UnityEngine;
using UnityEngine.UI;
using System.Collections;
using UnityEngine.Networking;
using Fusion.Photon.Realtime;
using System.Collections.Generic;
using System.Web;
using UnityEngine.SceneManagement;

public class ServerAPI : MonoBehaviour
{
    [SerializeField] private SceneReference _homeMenu;
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
                    var cookie_headers = HttpUtility.UrlDecode(response_headers["set-cookie"]);
                    SetPhotonAuthentication(cookie_headers);
                    
                    // SceneManager.LoadSceneAsync(_homeMenu);

                    loginButton.interactable = true;
                    progressCircle.SetActive(false);
                }
            }
        }

        www.Dispose();
    }

    void SetPhotonAuthentication(string cookie_headers)
    {
        // var sessionProps = _app.AutoSession;

        var authValues = new AuthenticationValues
        {
            AuthType = CustomAuthenticationType.Custom
        };

        var cookie_auth = "";
        var cookie_session = "";

        foreach (var part in cookie_headers.Split(","))
        {
            var equalsIndex = part.IndexOf('=');
            if (equalsIndex > 0)
            {
                var key = part.Substring(0, equalsIndex).Trim();
                var value = part.Substring(equalsIndex + 1).Split(';')[0];
                if (key == "session_token")
                {
                    cookie_session = value;
                } else if (key == "Authorization")
                {
                    cookie_auth = value.Replace("Bearer ", "");
                }
            }
        }

        Dictionary<string, object> body = new Dictionary<string, object>
        {
            ["auth_data"] = new Dictionary<string, string>
            {
                {"cookie_auth", cookie_auth},
                {"cookie_session", cookie_session}
            }
        };
        authValues.SetAuthPostData(body);

        _app.SetUserSession(cookie_headers);
        _app.SetAuthenticationValues(authValues);
        _app.InitiateGame();
    }
}
