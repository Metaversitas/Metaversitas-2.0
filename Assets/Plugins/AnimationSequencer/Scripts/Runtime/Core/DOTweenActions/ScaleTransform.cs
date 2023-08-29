﻿#if DOTWEEN_ENABLED
using System;
using DG.Tweening;
using DG.Tweening.Core;
using DG.Tweening.Plugins.Options;
using UnityEngine;

namespace BrunoMikoski.AnimationsSequencer
{
    [Serializable]
    public sealed class ScaleTransform : SequencerAnimationBase
    {
        public override Type TargetComponentType => typeof(Transform);
        public override string DisplayName => DisplayNames.ScaleTransform;

        [SerializeField]
        private Vector3 scale;
        public Vector3 Scale
        {
            get => scale;
            set => scale = value;
        }

        [SerializeField]
        private AxisConstraint axisConstraint;
        public AxisConstraint AxisConstraint
        {
            get => axisConstraint;
            set => axisConstraint = value;
        }

        private Vector3? previousState;
        private GameObject previousTarget;

        protected override Tweener GenerateTween_Internal(GameObject target, float duration)
        {
            previousState = target.transform.localScale;
            previousTarget = target;

            TweenerCore<Vector3, Vector3, VectorOptions> scaleTween =
                target.transform.DOScale(scale, duration).SetEase(ease);
            scaleTween.SetOptions(axisConstraint);

            return scaleTween;
        }

        public override void Reset()
        {
            if (!previousState.HasValue) 
                return;
            
            previousTarget.transform.localScale = previousState.Value;
        }
    }
}
#endif