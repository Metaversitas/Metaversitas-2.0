using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Fusion;

public class PhysicMenuSpawning : MonoBehaviour
{
    [SerializeField] private Character _character;
    public GameObject _notif;
    public GameObject _spawningMenu;

    public bool _isCollide;

    private void Start()
    {
        _notif.SetActive(false);
        _spawningMenu.SetActive(false);
    }
    private void OnTriggerEnter(Collider other)
    {
        if (other.tag == "Player")
        {
            _character = other.GetComponent<Character>();
           if (_character.Role == "Dosen")
            {
                _isCollide = true;
            }
        }
    }

    private void OnTriggerExit(Collider other)
    {
        if (other.tag == "Player")
        {
            _character = null;
            _isCollide = false;
        }
    }

    // Update is called once per frame
    void Update()
    {
        if(_isCollide) _notif.SetActive(true);
        else _notif.SetActive(false);

        if (Input.GetKeyDown("e"))
        {
            _spawningMenu.SetActive(true);
        }
        if (Input.GetKeyDown("escape")) { _spawningMenu.SetActive(false); }
    }
}
