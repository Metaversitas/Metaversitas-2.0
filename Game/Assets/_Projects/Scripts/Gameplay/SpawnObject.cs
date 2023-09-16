using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Fusion;
using System.Drawing;

public class SpawnObject : NetworkBehaviour
{
    [SerializeField] private PlayerStateManager _playerStateManager;
    [SerializeField] NetworkObject[] _prefabs;
    [SerializeField] Transform[] _points;

    [SerializeField] private NetworkObject[] spawnedObjects;
    // Indeks prefab yang akan di-spawn
    private int selectedPrefabIndex = 0;
    void Start()
    {
        // Inisialisasi array untuk melacak objek yang sudah di-spawn
        spawnedObjects = new NetworkObject[_points.Length];
    }
    public void AlatPraktikum(int i)
    {
        selectedPrefabIndex = i;
    }

    public void SpawnSelectedPrefab()
    {
        if (selectedPrefabIndex >= 0 && selectedPrefabIndex < _prefabs.Length)
        {
            var prefab = _prefabs[selectedPrefabIndex];

            for (int i = 0; i < _points.Length; i++)
            {
                // Cek jika ada objek yang sudah di-spawn di _points[i]
                if (spawnedObjects[i] != null)
                {
                    // Hancurkan objek yang sudah ada sebelum menggantinya
                    Runner.Despawn(spawnedObjects[i]);
                }

                // Spawn objek prefab baru dan simpan referensinya
                spawnedObjects[i] = Runner.Spawn(prefab, _points[i].position, _points[i].rotation);

                // Atur parent objek baru ke _points[i]
                if (spawnedObjects[i] != null)
                {
                    spawnedObjects[i].transform.SetParent(_points[i]);
                }
            }
        }
        _playerStateManager.TriggerPlayState();
    }
}
