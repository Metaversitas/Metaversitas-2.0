using ExitGames.Client.Photon;
using Fusion;
using Photon.Chat;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using TMPro;

public class PhotonChatManager : MonoBehaviour, IChatClientListener
{
    #region Setup

    [SerializeField] GameObject ChatView;
    [SerializeField] GameObject ChatBG;
    [SerializeField] GameObject PrivMessage;
    [SerializeField] GameObject joinChatButton;
    [SerializeField] private PlayerStateManager _playerStateManager;
    public ChatAppSettings ChatAppSettings
    {
        get { return this.chatAppSettings; }
    }

    [SerializeField]
    protected internal ChatAppSettings chatAppSettings;
    //[SerializeField] unameDisplay name;
    public ChatClient chatClient;
    public bool isConnected;
    public bool isChating;
    public bool privMessage;
    [SerializeField] string username;
    public string test;

    // Start is called before the first frame update
    public void UsernameOnValueChange(string valueIn)
    {
        username = valueIn;
    }

    public void ChatConnectOnClick()
    {
        // isConnected = true;
        // chatClient = new ChatClient(this);
        // //chatClient.ChatRegion = "US";
        // chatClient.Connect(PhotonNetwork.PhotonServerSettings.AppSettings.AppIdChat, PhotonNetwork.AppVersion,new AuthenticationValues(username));
        // Debug.Log("Connecting");
    }
    #endregion Setup

     #region General

    [SerializeField] GameObject chatPanel;
    string privateReceiver = "";
    string currentChat;
    [SerializeField] TMP_InputField chatField;
    [SerializeField] TMP_InputField privField;
    [SerializeField] TMP_Text chatDisplay;
    // [SerializeField] InputField chatField;
    // [SerializeField] Text chatDisplay;
    
    #endregion General
    
    #region PublicChat

    public void SubToChatOnClick()
    {
        chatClient.Subscribe(new string[] { "RegionChannel" });
    }
    public void TypePublicChatOnValueChange(string ChatIn)
    {
        currentChat = ChatIn;
    }
    public void SubmitPublicChatOnClick()
    {
        if (privateReceiver == "")
        {
            chatClient.PublishMessage("RegionChannel", currentChat);
            chatField.text = "";
            currentChat = "";
            _playerStateManager.TriggerPlayState();
        }
    }
    public void TypeChatOnValueChange(string valueIn)
    {
        currentChat = valueIn;   
    }

    #endregion PublicChat

    #region PrivateChat

    public void ReceiverOnValueChange(string valueIn)
    {
        privateReceiver = valueIn;
    }
    public void TypePrivateChatOnValueChange(string valueIn)
    {
        currentChat = valueIn;
    }
    public void SubmitPrivateChatOnClick()
    {
        if (privateReceiver != "")
        {
            chatClient.SendPrivateMessage(privateReceiver, currentChat);
            chatField.text = "";
            currentChat = "";
            _playerStateManager.TriggerPlayState();
        }
    }

    #endregion PrivateChat

    #region Callbacks

    public void DebugReturn(DebugLevel level, string message)
    {

    }

    public void OnChatStateChange(ChatState state)
    {




    }

    public void OnConnected()
    {
        Debug.Log("Connected");
        // isConnected = true;
        joinChatButton.SetActive(false);
        chatClient.Subscribe(new string[] { "RegionChannel" });
        // SubToChatOnClick();
    }

    public void OnDisconnected()
    {
        throw new System.NotImplementedException();
    }

    public void OnGetMessages(string channelName, string[] senders, object[] messages)
    {
        string msgs = "";
        for (int i = 0; i < senders.Length; i++)
        {
            msgs = string.Format("{0}: {1}", senders[i], messages[i]);

            chatDisplay.text += "\n " + msgs;

            Debug.Log(msgs);
        }
    }

    public void OnPrivateMessage(string sender, object message, string channelName)
    {
        string msgs = "";

        msgs = string.Format("(Private) {0}: {1}", sender, message);

        chatDisplay.text += "\n " + msgs;

        Debug.Log(msgs);
    }

    public void OnStatusUpdate(string user, int status, bool gotMessage, object message)
    {
        throw new System.NotImplementedException();    
    }

    public void OnSubscribed(string[] channels, bool[] results)
    {
        //chatPanel.SetActive(false);
    }

    public void OnUnsubscribed(string[] channels)
    {
        throw new System.NotImplementedException();
    }

    public void OnUserSubscribed(string channel, string user)
    {
        throw new System.NotImplementedException();
    }

    public void OnUserUnsubscribed(string channel, string user)
    {
        throw new System.NotImplementedException();     
    }
    #endregion callbacks


    void Start()
    {
        username=PlayerPrefs.GetString("username");
        isConnected = true;
        chatClient = new ChatClient(this);
        //chatClient.ChatRegion = "US";
        chatClient.ConnectUsingSettings(this.chatAppSettings);
        Debug.Log("Connecting");
    }
    void Update()
    {
        
        // chatClient.Service();
        if (isConnected)
        {
            chatClient.Service();
            
        }
        if (Input.GetKeyDown(KeyCode.Return) && !privMessage)
        {
            if (!isChating)
            {
                privField.DeactivateInputField();
                chatField.ActivateInputField();
                isChating = true;
                privMessage = false;
                ChatView.SetActive(true);
                ChatBG.SetActive(true);
                PrivMessage.SetActive(true);
                _playerStateManager.TriggerChattingState();
            }
            else if (chatField.text != "")
            {
                SubmitPublicChatOnClick();
                SubmitPrivateChatOnClick();
                _playerStateManager.TriggerPlayState();
            }
            else if (chatField.text == "")
            {
                chatField.DeactivateInputField();
                isChating = false;
                ChatView.SetActive(true);
                ChatBG.SetActive(false);
                _playerStateManager.TriggerPlayState();
            }
        }
        else if (Input.GetKeyDown(KeyCode.Return) && privMessage)
        {
            if (!isChating)
            {
                _playerStateManager.TriggerChattingState();
                privField.DeactivateInputField();
                chatField.ActivateInputField();
                isChating = true;
                privMessage = false;
                ChatView.SetActive(true);
                ChatBG.SetActive(false);
                PrivMessage.SetActive(true);
            }
            else
            {
                privField.DeactivateInputField();
                chatField.ActivateInputField();
                privMessage = false;
                _playerStateManager.TriggerPlayState();
            }
        }
        // Fungsi untuk private message menggunakan tab
        else if (Input.GetKeyDown(KeyCode.Tab))
        {
            if (!privMessage)
            {
                privMessage = true;
                chatField.DeactivateInputField();
                privField.ActivateInputField();
                _playerStateManager.TriggerChattingState();
            }
        }


    }
}
