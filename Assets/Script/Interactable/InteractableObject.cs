using System.Collections;
using System.Collections.Generic;
using UnityEditor.ShaderKeywordFilter;
using UnityEngine;
using Fusion;
using TPSBR;
using static UnityEngine.CullingGroup;

public class InteractableObject : NetworkBehaviour, IInteraction
{
    // HELPERS

    public enum EState
    {
        None,
        Closed,
        Open,
        Locked,
    }

    // PUBLIC MEMBERS
    [Networked(OnChanged = nameof(StateChanged), OnChangedTargets = OnChangedTargets.All), HideInInspector]
    public EState State { get; set; }

    [Header("Interaction")]
    [SerializeField]
    private string _interactionName;
    [SerializeField]
    private string _interactionDescription;
    [SerializeField]
    private Transform _hudPivot;
    [SerializeField]
    private Collider _interactionCollider;
    string IInteraction.Name => _interactionName;
    string  IInteraction.Description => _interactionDescription;
	Vector3 IInteraction.HUDPosition => _hudPivot != null ? _hudPivot.position : transform.position;
    bool IInteraction.IsActive => State == EState.Closed;
    public virtual void Interact()
    {
        Debug.Log(gameObject.name);
    }
}
