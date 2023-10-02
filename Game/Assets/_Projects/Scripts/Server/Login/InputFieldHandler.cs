using UnityEngine;
using UnityEngine.UI;

public class InputFieldHandler : MonoBehaviour
{
    // A reference to the input field component
    public InputField inputField;
    // A variable to store the input value
    private string inputValue;

    // A method to get the input value and assign it to the variable
    public string GetInputValue()
    {
        inputValue = inputField.text;
        Debug.Log("Input value: " + inputValue);
        return inputValue;
    }

    // A method to add an event listener to the input field
    private void Start()
    {
        // Add a listener to the input field that invokes GetInputValue() when the value changes
        inputField.onValueChanged.AddListener(delegate { GetInputValue(); });
    }
}
