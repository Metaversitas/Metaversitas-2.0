using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.InputSystem;
using Fusion;

public class PlayerInteractionHandler : NetworkBehaviour
{
    public void Interact()
    {
        float interactRange = 2f;
        Collider[] colliderArray = Physics.OverlapSphere(transform.position, interactRange);
        foreach (Collider collider in colliderArray)
        {
            if (collider.TryGetComponent(out InteractableObject interactableObject))
                interactableObject.Interact(); 
        }

    }
}
