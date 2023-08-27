using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CharacterCamera : MonoBehaviour
{
    [SerializeField]
    private Vector3 camPosition;

    [SerializeField]
    private Vector3 camRotation;

    private Transform _playerTransform;
    private Transform _mainCamera;

    private void Awake()
    {
        _playerTransform = transform;
        _mainCamera = Camera.main.transform;

        _mainCamera.position = Vector3.zero;
        _mainCamera.rotation = Quaternion.Euler(Vector3.zero);
        _mainCamera.SetParent(_playerTransform);

        // to optimize, just set once
        _mainCamera.localPosition = camPosition;
        _mainCamera.localRotation = Quaternion.Euler(camRotation);
    }

    // for tweaking purpose. uncomment the line and tweak during runtime
    void Update()
    {
        //_mainCamera.localPosition = camPosition;
        //_mainCamera.localRotation = Quaternion.Euler(camRotation);
    }
}
