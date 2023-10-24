using Metaversitas.User;
using UnityEngine;
using TMPro;
using System;

public class DisplayUserData : MonoBehaviour
{
    public App app;

    [SerializeField] private TMP_Text fullName;
    [SerializeField] private TMP_Text nim;
    [SerializeField] private TMP_Text facultyid;
    [SerializeField] private TMP_Text prodi;

    void Start()
    {
        app = App.FindInstance();

        // Try to get the user data.
        UserData userData;
        try
        {
            userData = app.get_userManager().get_userData();
        }
        catch (Exception e)
        {
            // Handle the exception.
            Debug.Log(e.Message);
            return;
        }

        // Convert the user data to TMPro.TMP_Text objects.
        fullName.SetText(userData.get_fullName());
        nim.SetText(userData.get_userUniversityID().ToString());
        facultyid.SetText(userData.get_facultyID().ToString());
        prodi.SetText(userData.get_facultyName());
    }
}
