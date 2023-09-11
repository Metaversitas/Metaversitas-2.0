using UnityEngine;
using System.Collections;
using Unity.VisualScripting;

[ExecuteInEditMode()]
public class GrappleRopeController : MonoBehaviour
{

    public Transform[] list_points;
    public LineRenderer lineRenderer;

	// Use this for initialization
	void Start () {
        lineRenderer.positionCount = list_points.Length;
	}

    private void Awake()
    {
        for (int i = 0; i < list_points.Length; i++)
		{
            lineRenderer.SetPosition(i, list_points[i].position);
        }
    }

    // Update is called once per frame
    void Update ()
    {
	    for (int i = 0; i < list_points.Length; ++i)
	    {
			
				lineRenderer.SetPosition(i, list_points[i].position);
	    }
    }
}