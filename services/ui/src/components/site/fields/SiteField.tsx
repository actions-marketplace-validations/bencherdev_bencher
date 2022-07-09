import FieldHelp from "./FieldHelp";
import SiteInput from "./form/SiteInput";
import SiteTextarea from "./form/SiteTextarea";
import SiteCheckbox from "./form/SiteCheckbox";
import SiteSwitch from "./form/SiteSwitch";
import SiteSelect from "./form/SiteSelect";

const SiteField = (props) => {
  function handleField(event, field = null) {
    switch (props.type) {
      case "checkbox":
        props.handleField(
          props.fieldKey,
          event.target.checked,
          event.target.checked
        );
        break;
      case "switch":
        props.handleField(props.fieldKey, event.target.checked, true);
        break;
      case "select":
        props.handleField(
          props.fieldKey,
          { ...props.value, selected: event.target.value },
          true
        );
        break;
      default:
        props.handleField(
          props.fieldKey,
          event.target.value,
          props.config.validate
            ? props.config.validate(event.target.value)
            : true
        );
    }
  }

  function getField() {
    switch (props.type) {
      case "textarea":
      case "code":
        return (
          <SiteTextarea
            value={props.value}
            config={props.config}
            handleField={handleField}
          />
        );
      case "checkbox":
        return (
          <SiteCheckbox
            value={props.value}
            config={props.config}
            handleField={handleField}
          />
        );
      case "switch":
        return (
          <SiteSwitch
            value={props.value}
            config={props.config}
            handleField={handleField}
          />
        );
      case "select":
        return (
          <SiteSelect
            value={props.value}
            config={props.config}
            handleField={handleField}
          />
        );
      default:
        return (
          <SiteInput
            value={props.value}
            valid={props.valid}
            config={props.config}
            handleField={handleField}
          />
        );
    }
  }

  function getValidate() {
    switch (props.type) {
      case "checkbox":
      case "switch":
      case "select":
        return false;
      default:
        return true;
    }
  }

  return (
    <div class="field">
      {props.label && props.config && props.config.label && (
        <label class="label is-medium">{props.config.label}</label>
      )}
      {getField()}
      {getValidate() && props.valid === false && (
        <FieldHelp fieldText={props.config.help} fieldValid={props.valid} />
      )}
    </div>
  );
};

export default SiteField;