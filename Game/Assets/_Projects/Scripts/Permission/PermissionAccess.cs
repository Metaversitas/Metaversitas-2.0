using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Metaversitas.User;
using UnityEngine.UI;

public class PermissionAccess : MonoBehaviour
{
    [SerializeField] private GameObject sessionButton;
    public App app;
    public string Role;

    void Start()
    {
        app = App.FindInstance();
        UserData userData = app.get_userManager().get_userData();

        if (userData.get_userRole() == UserUniversityRole.Dosen)
        {
            Debug.Log("Membuat Session");
            sessionButton.SetActive(true);
        }
        else
        {
            sessionButton.SetActive(false);
            Debug.Log("Tidak Dapar Membuat Session");
        }
    }

}

