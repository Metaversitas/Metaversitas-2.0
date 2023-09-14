using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Fusion;
using System.Drawing;

public class SpawnObject : NetworkBehaviour
{
    [SerializeField] GameObject _prefabs;
    public Transform point;
    
    // Start is called before the first frame update
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        
    }

    public void SpawningObject()
    {
       var prefab = _prefabs;
       var pickup = Runner.Spawn(prefab, point.position, point.rotation);
    }
}
