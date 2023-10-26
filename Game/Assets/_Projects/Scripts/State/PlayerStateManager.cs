using System;
using UnityEngine;

public class PlayerStateManager : MonoBehaviour
{
    public event EventHandler PlayState;
    public event EventHandler PauseState;
    public event EventHandler ChattingState;
    public event EventHandler InteractState;
    public event EventHandler IntroState;
    public event EventHandler EndState;


    public GameState CurrentGameState { get; private set; } = GameState.Play;
    public bool IsVideoPlaying { get; internal set; }

    // Method to change the game state
    public void SetGameState(GameState newState)
    {
        CurrentGameState = newState;
    }


    // Call this method to trigger the play state event
    public void TriggerPlayState()
    {
        PlayState?.Invoke(this, EventArgs.Empty);
        SetGameState(GameState.Play);
    }

    public void TriggerIntroState()
    {
        IntroState?.Invoke(this, EventArgs.Empty);
        SetGameState(GameState.Intro);
    }

    // Call this method to trigger the pause state event
    public void TriggerPauseState()
    {
        PauseState?.Invoke(this, EventArgs.Empty);
        SetGameState(GameState.Pause);
    }

    public void TriggerChattingState()
    {
        ChattingState?.Invoke(this, EventArgs.Empty);
        SetGameState(GameState.Chatting);
    }

    public void TriggerInteractState()
    {
        InteractState?.Invoke(this, EventArgs.Empty);
        SetGameState(GameState.Interact);
    }

    // Call this method to trigger the end state event
    public void TriggerEndState()
    {
        EndState?.Invoke(this, EventArgs.Empty);
        SetGameState(GameState.End);
    }
}