using System;
using System.Collections;
using UnityEngine;
using UnityEngine.Networking;

namespace Metaversitas.User
{
    [Serializable]
    public class UserSession: MonoBehaviour
    {
        private bool _isInitialized = false;
        private const int FourMinutes = 60 * 4;
        private const string UriRefreshToken = "https://metaversitas.rweebz.xyz/auth/refresh";
        private const string CookieAuthorization = "Authorization";
        private const string CookieSession = "session_token";
        private string _cookieValue;

        public void Initialize(string cookieValue)
        {
            if (_isInitialized)
            {
                Debug.Log("User Session is already initialized");
                return;
            }
            
            _cookieValue = cookieValue;
            StartCoroutine(RefreshSession());
        }
        
        private IEnumerator RefreshSession()
        {
            while (true)
            {
                yield return new WaitForSeconds(FourMinutes);
                
                using var request = UnityWebRequest.Get(UriRefreshToken);
                request.SetRequestHeader("Cookie", _cookieValue);
                yield return request.SendWebRequest();
    
                if (request.responseCode != 200)
                {
                    throw new SystemException("Invalid data form server");
                }

                var cookieValue = request.GetResponseHeader("set-cookie");
                _cookieValue = cookieValue;
            }   
            
        }
    }
}