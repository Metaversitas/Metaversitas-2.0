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
        var email = usernameField.text;
        var password = passwordField.text;

        Dictionary<string, object> body = new Dictionary<string, object>
        {
            ["user"] = new Dictionary<string, string>
            {
                {"email", email},
                {"password", password}
            }
        };

        var sessionProps = _app.AutoSession;
        var authValues = new AuthenticationValues
        {
            AuthType = CustomAuthenticationType.Custom
        };
        authValues.SetAuthPostData(body);

        _app.SetAuthenticationValues(authValues);
        _app.InitiateGame();
    }
}
