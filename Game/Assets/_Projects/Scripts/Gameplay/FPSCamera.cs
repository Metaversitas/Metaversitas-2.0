using Fusion;
using System.Collections;
using UnityEngine;

public class FPSCamera : NetworkBehaviour, IBeforeUpdate
{
    public float mouseSensitivity = 2f;
    public float maxYDegrees = 70f;
    public Character Character;
    public Angle yCamRotDelta;

    [SerializeField]
    private Vector3 camPosition;

    private CursorLock _cursorLock;
    private float verticalRotation = 0;
    private Transform _camTransform;

    public override void Spawned()
    {
        _cursorLock = GetComponent<CursorLock>();
        _cursorLock.ToggleCursorLock();
        _camTransform = Camera.main.transform;

        if (Object.HasInputAuthority)
        {
            SetCameraParent(Character.transform);
            App.FindInstance().FpsCamera = this;
        }
    }

    public void SetCameraParent(Transform parent)
    {
        _camTransform.position = Vector3.zero;
        _camTransform.rotation = Quaternion.Euler(Vector3.zero);

        _camTransform.SetParent(parent);

        _camTransform.localPosition = camPosition;
        _camTransform.localRotation = Quaternion.Euler(Vector3.zero);
    }

    public override void Render()
    {
        if (Character == null) return;

        Character.transform.rotation = Quaternion.Euler(0, (float)Character.yCamRotation, 0);
    }

    private void Update()
    {
        if (Object.HasInputAuthority == false) return;
        if (_cursorLock.IsLocked == false) return;
        ControlCameraUsingMouse();
    }

    private void ControlCameraUsingMouse()
    {
        float mouseY = Input.GetAxis("Mouse Y") * mouseSensitivity;

        verticalRotation -= mouseY;
        verticalRotation = Mathf.Clamp(verticalRotation, -maxYDegrees, maxYDegrees);

        _camTransform.localRotation = Quaternion.Euler(verticalRotation, _camTransform.localEulerAngles.y, 0);
    }

    public void BeforeUpdate()
    {
        if (_cursorLock.IsLocked == false) return;
        AccumulateDelta();
    }

    private void AccumulateDelta()
    {
        yCamRotDelta += Input.GetAxis("Mouse X");
    }

    public Angle ConsumeDelta()
    {
        var consumed = yCamRotDelta;
        yCamRotDelta = 0;
        return consumed;
    }
}