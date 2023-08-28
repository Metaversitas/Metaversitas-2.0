using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CharacterCamera : MonoBehaviour
{
    [SerializeField]
    private Vector3 camPosition;

    [SerializeField]
    private Vector3 camRotation;

    private Transform _mainCamera;

    public void SetCameraParent(Transform parent)
    {
        _mainCamera = Camera.main.transform;

        _mainCamera.position = Vector3.zero;
        _mainCamera.rotation = Quaternion.Euler(Vector3.zero);

        _mainCamera.SetParent(parent);

        // to optimize, just set once
        _mainCamera.localPosition = camPosition;
        _mainCamera.localRotation = Quaternion.Euler(_mainCamera.localEulerAngles + camRotation);
    }

    // for tweaking purpose. uncomment the line and tweak during runtime
    private void LateUpdate()
    {
        //_mainCamera.localPosition = camPosition;
        //_mainCamera.localRotation = Quaternion.Euler(camRotation);
    }
}
