using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using TMPro;
using static System.Net.Mime.MediaTypeNames;

public class Meter : MonoBehaviour
{
    [SerializeField] Transform point;
    [SerializeField] TextMeshProUGUI Atext;
    public Vector3 Startposition;
    public Rigidbody rb;
    public HingeJoint hinge; 
    public float distance = 1.0f;
    public float curangle;
    public float maxAngle;
    public float minAngle;
    public float newangle;
    private bool isDragging = false;
    private Vector3 offset;
    // Start is called before the first frame update
    void Start()
    {
        rb = GetComponent<Rigidbody>();
        gameObject.transform.position = Startposition;
        hinge.connectedAnchor = Startposition;
    }

    // Update is called once per frame
    void Update()
    {
        if (isDragging)
        {
            rb.drag = 0f;
            // angle 
            curangle = transform.rotation.eulerAngles.x;
            CalculateAngle();
            Atext.text = curangle.ToString("f1");
            // distance
            distance = (point.transform.position - transform.position).magnitude;
            // Memperbarui posisi objek berdasarkan pergerakan mouse
            transform.position = GetMouseWorldPos() + offset;
        }

        if (Input.GetKeyDown("1"))
        {
            MoveObjectToValue(4);
        }
        else if (Input.GetKeyDown("2"))
        {
            MoveObjectToValue(6);
        }
        else if (Input.GetKeyDown("3"))
        {
            MoveObjectToValue(8);
        }

    }

    void MoveObjectToValue(float targetValue)
    {
        Vector3 newAnchor = new Vector3(hinge.connectedAnchor.x, targetValue, hinge.connectedAnchor.z);
        hinge.connectedAnchor = newAnchor;
    }

    private void OnMouseDown()
    {
        isDragging = true;
        // Menghitung perbedaan antara posisi mouse dan posisi objek
        offset = transform.position - GetMouseWorldPos();
    }

    private void OnMouseUp()
    {
        isDragging = false;
    }

    private Vector3 GetMouseWorldPos()
    {
        // Mengubah posisi mouse dari layar ke koordinat dunia (world coordinates)
        Vector3 mousePoint = Input.mousePosition;
        mousePoint.z = Camera.main.transform.position.z;

        return Camera.main.ScreenToWorldPoint(mousePoint);
    }

    void OnMouseExit()
    {
        StartCoroutine("ClearAngle");

    }
    void ClampRotation()
    {
        newangle = Mathf.Clamp (curangle, minAngle, maxAngle);
            
        transform.rotation = Quaternion.Euler (new Vector3(transform.rotation.eulerAngles.x,transform.rotation.eulerAngles.y,newangle));
    }

    public void reset()
    {   
        rb.drag = 50f;
        hinge.connectedAnchor = Startposition;
    }
    void CalculateAngle ()
    {
        if (curangle > 90 && curangle < 180 )
        {
            curangle = 90 - curangle;   
        } 
        else if (curangle > 180 && curangle < 270 ) 
        {
            curangle = 180 - curangle;
        } 
        else if (curangle > 270 && curangle <= 360 ) 
        {
            curangle = 360 - curangle;
        }

        if (curangle > 90 || curangle < 0)
        {
            ClampRotation ();
        }
    }

    IEnumerator ClearAngle()
    {
        yield return new WaitForSeconds(1.0f);
    }
}
