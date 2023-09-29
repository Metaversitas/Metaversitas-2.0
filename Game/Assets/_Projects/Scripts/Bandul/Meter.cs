using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using TMPro;
using static System.Net.Mime.MediaTypeNames;
using Fusion;
using UnityEngine.InputSystem;

public class Meter : NetworkBehaviour
{
    [SerializeField] Transform point;
    [SerializeField] TextMeshProUGUI Atext;
    public Vector3 Startposition;
    public Rigidbody rb;
    public HingeJoint hinge;
    private UIPraktikum _uIPraktikum;
    private CheckInteractionNetwork _checkInteractionNetwork;
    public float distance = 1.0f;
    public float curangle;
    public float maxAngle;
    public float minAngle;
    public float newangle;
    public float delayInteract = 1f;
    public bool isDragging = false;
    private Vector3 offset;

    [Networked]
    private TickTimer delay { get; set; }

    public override void Spawned()
    {
        delay = TickTimer.CreateFromSeconds(Runner, delayInteract);
    }
    // Start is called before the first frame update
    void Start()
    {
        _uIPraktikum = GetComponentInParent<UIPraktikum>();
        _checkInteractionNetwork = GetComponentInParent<CheckInteractionNetwork>();
        rb = GetComponent<Rigidbody>();
        gameObject.transform.position = Startposition;
        hinge.connectedAnchor = Startposition;
    }

    public void MouseClick()
    {
        RaycastHit hit;
        Ray ray = Camera.main.ScreenPointToRay(Input.mousePosition);

        if (Physics.Raycast(ray, out hit))
        {
            if (hit.collider.gameObject == gameObject)
            {
                // Objek ini diklik
                Debug.Log("Mouse Down on Object");
                RPC_OnMouseDown();
            }
        }
    }

    [Rpc]
    public void RPC_OnMouseDown()
    {
        MousePosition();
    }

    public void MousePosition()
    {
        isDragging = true;
        // Menghitung perbedaan antara posisi mouse dan posisi objek
        offset = transform.position - GetMouseWorldPos();
    }

    [Rpc]
    public void RPC_MousePositionUP()
    {
        MousePositionUP();
    }
    public void MousePositionUP() { isDragging = false; }

    public void OnMouseExit()
    {
        Rpc_StartCoroutine();
    }

        [Rpc]
    public void Rpc_StartCoroutine()
    {
        StartCoroutine("ClearAngle");
    }
    private Vector3 GetMouseWorldPos()
    {
        // Mengubah posisi mouse dari layar ke koordinat dunia (world coordinates)
        Vector3 mousePoint = Input.mousePosition;
        mousePoint.z = Camera.main.transform.position.z;

        return Camera.main.ScreenToWorldPoint(mousePoint);
    }

    [Rpc]
    public void RPC_Reset()
    {
        reset();
    }

    public void reset()
    {
        rb.drag = 50f;
        hinge.connectedAnchor = Startposition;
    }

    [Rpc]
    void RPC_MoveObjectToValue(float targetValue)
    {
        MoveObjectToValue(targetValue);
    }

    void MoveObjectToValue(float targetValue)
    {
        Vector3 newAnchor = new Vector3(hinge.connectedAnchor.x, targetValue, hinge.connectedAnchor.z);
        hinge.connectedAnchor = newAnchor;
    }

    void ClampRotation()
    {
        newangle = Mathf.Clamp(curangle, minAngle, maxAngle);

        transform.rotation = Quaternion.Euler(new Vector3(transform.rotation.eulerAngles.x, transform.rotation.eulerAngles.y, newangle));
    }

    void CalculateAngle()
    {
        if (curangle > 90 && curangle < 180)
        {
            curangle = 90 - curangle;
        }
        else if (curangle > 180 && curangle < 270)
        {
            curangle = 180 - curangle;
        }
        else if (curangle > 270 && curangle <= 360)
        {
            curangle = 360 - curangle;
        }

        if (curangle > 90 || curangle < 0)
        {
            ClampRotation();
        }
    }

    IEnumerator ClearAngle()
    {
        yield return new WaitForSeconds(1.0f);
    }

    // Update is called once per frame
    void Update()
    {
        Angle();
        if (isDragging)
        {
            rb.drag = 0f;
            // distance
            distance = (point.transform.position - transform.position).magnitude;
            // Memperbarui posisi objek berdasarkan pergerakan mouse
            transform.position = GetMouseWorldPos() + offset;
        }

        if (Input.GetKeyDown("1"))
        {
            RPC_MoveObjectToValue(4);
        }
        else if (Input.GetKeyDown("2"))
        {
            RPC_MoveObjectToValue(6);
        }
        else if (Input.GetKeyDown("3"))
        {
            RPC_MoveObjectToValue(8);
        }
    }

    private void Angle()
    {
        // angle 
        curangle = transform.rotation.eulerAngles.x;
        if (curangle > 0.5)
        {
            CalculateAngle();
            Atext.text = curangle.ToString("f1");
        }
    }

    
}
