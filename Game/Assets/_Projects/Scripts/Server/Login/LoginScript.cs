using UnityEngine;
using UnityEngine.UI;
using System.Collections;
using System.Collections.Generic;
using Fusion.Photon.Realtime;
using UnityEngine.SceneManagement;

public class LoginScript : MonoBehaviour
{
    [SerializeField] private InputField usernameField;
    [SerializeField] private InputField passwordField;
    [SerializeField] private SceneReference sceneToLoad;

    private App _app;

    void Start()
    {
        _app = App.FindInstance();
        
        if (_app == null)
        {
            Debug.LogError("Instance of App not found!");
        }
    }

    public void Login()
    {
        string email = usernameField.text;
        string password = passwordField.text;

        Dictionary<string, object> body = new Dictionary<string, object>();
        body["user"] = new Dictionary<string, string>
        {
            {"email", email},
            {"password", password}
        };

        SessionProps sessionProps = _app.AutoSession;
        AuthenticationValues authValues = new AuthenticationValues
        {
            AuthType = CustomAuthenticationType.Custom,
        };
        authValues.SetAuthPostData(body);

        _app.SetAuthenticationValues(authValues);
        /*_app.CreateSession(sessionProps);*/
        SceneManager.LoadSceneAsync(sceneToLoad);
    }
}
